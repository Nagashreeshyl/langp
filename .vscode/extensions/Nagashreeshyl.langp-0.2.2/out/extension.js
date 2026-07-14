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
const langp_api_1 = require("./langp-api");
let client;
const diagnosticCollection = vscode.languages.createDiagnosticCollection("langp");
const output = vscode.window.createOutputChannel("Lang.P");
let checkTimer;
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
            const loc = lines[i + 1]?.match(/-->\s*[^:]+:(\d+):(\d+)/);
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
    if (!fs.existsSync(lang))
        return;
    const tmp = path.join(os.tmpdir(), `langp-check-${Date.now()}.lp`);
    try {
        fs.writeFileSync(tmp, doc.getText());
        const result = cp.spawnSync(lang, ["check", tmp], { encoding: "utf8" });
        const combined = `${result.stdout}\n${result.stderr}`;
        if (result.status === 0 && !combined.includes("error[")) {
            diagnosticCollection.set(doc.uri, []);
            return;
        }
        diagnosticCollection.set(doc.uri, parseDiagnostics(combined, doc));
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
function scheduleCheck(doc) {
    const cfg = vscode.workspace.getConfiguration("langp");
    if (!cfg.get("checkOnType", true))
        return;
    if (checkTimer)
        clearTimeout(checkTimer);
    checkTimer = setTimeout(() => runCheck(doc), 400);
}
function registerIntelliSense(context) {
    const selector = { language: "langp", scheme: "file" };
    context.subscriptions.push(vscode.languages.registerCompletionItemProvider(selector, {
        provideCompletionItems(doc, position) {
            const prefix = (0, langp_api_1.wordPrefix)(doc, position);
            const line = doc.lineAt(position.line).text;
            const before = line.slice(0, position.character);
            // After "input " suggest types (text, number, …)
            if (/\binput\s+$/.test(before) || /\binput\s+\w*$/.test(before)) {
                const typePrefix = before.match(/\binput\s+(\w*)$/)?.[1] ?? "";
                return langp_api_1.INPUT_TYPES.filter((t) => (0, langp_api_1.matchesPrefix)(t, typePrefix)).map((t) => {
                    const item = new vscode.CompletionItem(t, vscode.CompletionItemKind.EnumMember);
                    item.detail = `input ${t} "prompt"`;
                    item.documentation = new vscode.MarkdownString(`Input type for \`input ${t}\``);
                    item.insertText = new vscode.SnippetString(`${t} "\${1:prompt}"`);
                    return item;
                });
            }
            const items = [];
            for (const entry of langp_api_1.BUILTINS) {
                if ((0, langp_api_1.matchesPrefix)(entry.name, prefix)) {
                    items.push((0, langp_api_1.entryToCompletion)(entry, prefix));
                }
            }
            for (const entry of langp_api_1.KEYWORDS) {
                if ((0, langp_api_1.matchesPrefix)(entry.name, prefix)) {
                    items.push((0, langp_api_1.entryToCompletion)(entry, prefix));
                }
            }
            // Snippets only when user typed at least 1 char or invoked manually
            if (prefix.length >= 1) {
                const snippets = [
                    { label: "if block", kind: vscode.CompletionItemKind.Snippet, sig: "if condition, … ..", text: "if ${1:condition},\n\t${2:pass}\n.." },
                    { label: "repeat", kind: vscode.CompletionItemKind.Snippet, sig: "repeat N times, … ..", text: "repeat ${1:5} times,\n\t${2:pass}\n.." },
                    { label: "for", kind: vscode.CompletionItemKind.Snippet, sig: "for item in items, … ..", text: "for ${1:item} in ${2:items},\n\t${3:pass}\n.." },
                ];
                for (const s of snippets) {
                    if ((0, langp_api_1.matchesPrefix)(s.label, prefix) || (0, langp_api_1.matchesPrefix)(s.label.replace(" block", ""), prefix)) {
                        const item = new vscode.CompletionItem(s.label, s.kind);
                        item.detail = s.sig;
                        item.insertText = new vscode.SnippetString(s.text);
                        item.sortText = `2_${s.label}`;
                        items.push(item);
                    }
                }
            }
            return new vscode.CompletionList(items, false);
        },
    }, ".", " ", "(", ",", "@"), vscode.languages.registerSignatureHelpProvider(selector, {
        provideSignatureHelp(doc, position) {
            const line = doc.lineAt(position.line).text.slice(0, position.character);
            const fnMatch = line.match(/(\w+)\s*\(?[^()]*$/);
            if (!fnMatch)
                return null;
            const name = fnMatch[1];
            const entry = (0, langp_api_1.findEntry)(name);
            if (!entry)
                return null;
            const sig = new vscode.SignatureInformation(entry.signature, new vscode.MarkdownString(entry.doc));
            return {
                signatures: [sig],
                activeSignature: 0,
                activeParameter: 0,
            };
        },
    }, "(", ",", " "), vscode.languages.registerHoverProvider(selector, {
        provideHover(doc, position) {
            const prefix = (0, langp_api_1.wordPrefix)(doc, position);
            if (!prefix)
                return null;
            const entry = (0, langp_api_1.findEntry)(prefix);
            if (!entry)
                return null;
            return new vscode.Hover(new vscode.MarkdownString(`**${entry.name}**\n\n\`${entry.signature}\`\n\n${entry.doc}`));
        },
    }));
}
function startLanguageServer() {
    const langpConfig = vscode.workspace.getConfiguration("langp");
    if (!langpConfig.get("enableLanguageServer", true))
        return;
    const configured = langpConfig.get("languageServerPath", "").trim();
    const serverPath = configured || binPath("lang-lsp");
    if (!fs.existsSync(serverPath)) {
        output.appendLine("lang-lsp not found — using built-in IntelliSense");
        return;
    }
    const serverOptions = {
        run: { command: serverPath, args: [], transport: node_1.TransportKind.stdio },
        debug: { command: serverPath, args: [], transport: node_1.TransportKind.stdio },
    };
    const clientOptions = {
        documentSelector: [{ scheme: "file", language: "langp", pattern: "**/*.lp" }],
        synchronize: { fileEvents: vscode.workspace.createFileSystemWatcher("**/*.lp") },
        outputChannel: output,
    };
    client = new node_1.LanguageClient("langp-lsp", "Lang.P LSP", serverOptions, clientOptions);
    void client.start().then(() => output.appendLine("lang-lsp connected"), (e) => output.appendLine(`lang-lsp failed: ${e}`));
}
async function activate(context) {
    output.appendLine("Lang.P IntelliSense activated");
    const status = vscode.window.createStatusBarItem(vscode.StatusBarAlignment.Right, 100);
    status.text = "$(symbol-method) Lang.P";
    status.tooltip = "Lang.P — IntelliSense active (not AI suggestions)";
    status.command = "langp.setLanguage";
    status.show();
    registerIntelliSense(context);
    for (const doc of vscode.workspace.textDocuments) {
        await forceLangpLanguage(doc);
        runCheck(doc);
    }
    context.subscriptions.push(diagnosticCollection, output, status, vscode.workspace.onDidOpenTextDocument(async (doc) => {
        await forceLangpLanguage(doc);
        runCheck(doc);
    }), vscode.workspace.onDidSaveTextDocument((doc) => runCheck(doc)), vscode.workspace.onDidChangeTextDocument((e) => {
        if (e.document.fileName.endsWith(".lp"))
            scheduleCheck(e.document);
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
    }), vscode.commands.registerCommand("langp.checkFile", () => {
        const editor = vscode.window.activeTextEditor;
        if (editor)
            runCheck(editor.document);
    }), vscode.commands.registerCommand("langp.setLanguage", async () => {
        const editor = vscode.window.activeTextEditor;
        if (editor?.document.fileName.endsWith(".lp")) {
            await forceLangpLanguage(editor.document);
        }
    }));
    startLanguageServer();
}
async function deactivate() {
    if (checkTimer)
        clearTimeout(checkTimer);
    diagnosticCollection.dispose();
    if (client)
        await client.stop();
}
//# sourceMappingURL=extension.js.map