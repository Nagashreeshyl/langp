"use strict";
var __createBinding = (this && this.__createBinding) || (Object.create ? (function(o, m, k, k2) {
    if (k2 === undefined) k2 = k;
    var desc = Object.getOwnPropertyDescriptor(m, k);
    if (!desc || ("get" in desc ? !m.__esModule : desc.writable || desc.configurable)) {
      desc = { enumerable: true, get: function() { return m[k]; } };
    }
    Object.defineProperty(o, k2, desc);
}) : (function(o, m, k, k2) {
    if (k2 === undefined) k2 = k;
    o[k2] = m[k];
}));
var __setModuleDefault = (this && this.__setModuleDefault) || (Object.create ? (function(o, v) {
    Object.defineProperty(o, "default", { enumerable: true, value: v });
}) : function(o, v) {
    o["default"] = v;
});
var __importStar = (this && this.__importStar) || (function () {
    var ownKeys = function(o) {
        ownKeys = Object.getOwnPropertyNames || function (o) {
            var ar = [];
            for (var k in o) if (Object.prototype.hasOwnProperty.call(o, k)) ar[ar.length] = k;
            return ar;
        };
        return ownKeys(o);
    };
    return function (mod) {
        if (mod && mod.__esModule) return mod;
        var result = {};
        if (mod != null) for (var k = ownKeys(mod), i = 0; i < k.length; i++) if (k[i] !== "default") __createBinding(result, mod, k[i]);
        __setModuleDefault(result, mod);
        return result;
    };
})();
Object.defineProperty(exports, "__esModule", { value: true });
exports.activate = activate;
exports.deactivate = deactivate;
const cp = __importStar(require("child_process"));
const fs = __importStar(require("fs"));
const os = __importStar(require("os"));
const path = __importStar(require("path"));
const vscode = __importStar(require("vscode"));
const node_1 = require("vscode-languageclient/node");
let client;
const diagnosticCollection = vscode.languages.createDiagnosticCollection("langp");
const output = vscode.window.createOutputChannel("Lang.P");
function binPath(name) {
    const home = os.homedir();
    const candidates = [
        path.join(home, ".local", "bin", name),
        path.join(home, ".cargo", "bin", name),
    ];
    if (process.env.PATH) {
        for (const dir of process.env.PATH.split(path.delimiter)) {
            candidates.push(path.join(dir, name));
        }
    }
    for (const c of candidates) {
        if (fs.existsSync(c))
            return c;
    }
    return path.join(home, ".local", "bin", name);
}
async function forceLangpLanguage(doc) {
    if (doc.uri.scheme !== "file")
        return;
    if (!doc.fileName.endsWith(".lp"))
        return;
    if (doc.languageId === "langp")
        return;
    try {
        await vscode.languages.setTextDocumentLanguage(doc, "langp");
        output.appendLine(`Set language to langp: ${doc.fileName}`);
    }
    catch (e) {
        output.appendLine(`Failed to set language: ${e}`);
    }
}
function parseDiagnostics(text, doc) {
    const diags = [];
    const lines = text.split("\n");
    let i = 0;
    while (i < lines.length) {
        const line = lines[i];
        const errMatch = line.match(/^(error|warning)\[([^\]]+)\]:\s*(.+)$/i);
        if (errMatch) {
            const severity = errMatch[1].toLowerCase() === "warning"
                ? vscode.DiagnosticSeverity.Warning
                : vscode.DiagnosticSeverity.Error;
            const message = errMatch[3];
            const code = errMatch[2];
            let lineNum = 0;
            let col = 0;
            const loc = lines[i + 1]?.match(/-->\s*source:(\d+):(\d+)/);
            if (loc) {
                lineNum = Math.max(0, parseInt(loc[1], 10) - 1);
                col = Math.max(0, parseInt(loc[2], 10) - 1);
                i += 1;
            }
            const docLine = doc.lineAt(Math.min(lineNum, doc.lineCount - 1));
            const start = Math.min(col, docLine.text.length);
            const end = Math.min(start + 1, docLine.text.length);
            diags.push({
                range: new vscode.Range(lineNum, start, lineNum, Math.max(end, start + 1)),
                message,
                severity,
                source: "langp",
                code,
            });
        }
        i += 1;
    }
    return diags;
}
function runCheck(doc) {
    if (doc.languageId !== "langp" && !doc.fileName.endsWith(".lp"))
        return;
    const lang = binPath("lang");
    if (!fs.existsSync(lang)) {
        output.appendLine(`lang not found at ${lang}`);
        return;
    }
    const tmp = path.join(os.tmpdir(), `langp-check-${Date.now()}.lp`);
    try {
        fs.writeFileSync(tmp, doc.getText());
        const result = cp.spawnSync(lang, ["check", tmp], { encoding: "utf8" });
        const combined = `${result.stdout}\n${result.stderr}`;
        if (result.status === 0 && !combined.includes("error[")) {
            diagnosticCollection.set(doc.uri, []);
            return;
        }
        const diags = parseDiagnostics(combined, doc);
        diagnosticCollection.set(doc.uri, diags);
    }
    catch (e) {
        output.appendLine(`check failed: ${e}`);
    }
    finally {
        try {
            fs.unlinkSync(tmp);
        }
        catch {
            /* ignore */
        }
    }
}
function startLanguageServer() {
    const langpConfig = vscode.workspace.getConfiguration("langp");
    if (!langpConfig.get("enableLanguageServer", true))
        return;
    const configured = langpConfig.get("languageServerPath", "").trim();
    const serverPath = configured || binPath("lang-lsp");
    if (!fs.existsSync(serverPath)) {
        output.appendLine(`lang-lsp not found at ${serverPath} — using built-in checker`);
        return;
    }
    const serverOptions = {
        run: { command: serverPath, args: [], transport: node_1.TransportKind.stdio },
        debug: { command: serverPath, args: [], transport: node_1.TransportKind.stdio },
    };
    const clientOptions = {
        documentSelector: [{ scheme: "file", pattern: "**/*.lp" }],
        synchronize: { fileEvents: vscode.workspace.createFileSystemWatcher("**/*.lp") },
        outputChannel: output,
    };
    client = new node_1.LanguageClient("langp-lsp", "Lang.P LSP", serverOptions, clientOptions);
    void client.start();
}
async function activate(context) {
    output.appendLine("Lang.P extension activated");
    for (const doc of vscode.workspace.textDocuments) {
        await forceLangpLanguage(doc);
        runCheck(doc);
    }
    context.subscriptions.push(diagnosticCollection, output, vscode.workspace.onDidOpenTextDocument(async (doc) => {
        await forceLangpLanguage(doc);
        runCheck(doc);
    }), vscode.workspace.onDidSaveTextDocument((doc) => runCheck(doc)), vscode.workspace.onDidChangeTextDocument((e) => {
        if (e.document.fileName.endsWith(".lp")) {
            runCheck(e.document);
        }
    }), vscode.window.onDidChangeActiveTextEditor(async (editor) => {
        if (editor) {
            await forceLangpLanguage(editor.document);
            runCheck(editor.document);
        }
    }), vscode.commands.registerCommand("langp.runFile", () => {
        const editor = vscode.window.activeTextEditor;
        if (!editor?.document.fileName.endsWith(".lp"))
            return;
        const lang = binPath("lang");
        const term = vscode.window.createTerminal("Lang.P");
        term.show();
        term.sendText(`${lang} run "${editor.document.fileName}"`);
    }));
    startLanguageServer();
    void vscode.window.showInformationMessage("Lang.P language support active");
}
async function deactivate() {
    diagnosticCollection.dispose();
    if (client)
        await client.stop();
}
//# sourceMappingURL=extension.js.map