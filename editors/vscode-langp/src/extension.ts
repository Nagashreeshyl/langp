import * as cp from "child_process";
import * as fs from "fs";
import * as os from "os";
import * as path from "path";
import * as vscode from "vscode";
import {
  LanguageClient,
  LanguageClientOptions,
  ServerOptions,
  TransportKind,
} from "vscode-languageclient/node";
import {
  BUILTINS,
  INPUT_TYPES,
  KEYWORDS,
  SNIPPETS,
  STATEMENTS,
  entryToCompletion,
  findEntry,
  matchesPrefix,
  wordPrefix,
} from "./langp-api";
import { variableCompletions } from "./variables";

let client: LanguageClient | undefined;
const diagnosticCollection = vscode.languages.createDiagnosticCollection("langp");
const output = vscode.window.createOutputChannel("Lang.P");
let checkTimer: ReturnType<typeof setTimeout> | undefined;

function binPath(name: string): string {
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
    if (fs.existsSync(c)) return c;
  }
  return path.join(home, ".local", "bin", name);
}

async function forceLangpLanguage(doc: vscode.TextDocument): Promise<void> {
  if (doc.uri.scheme !== "file") return;
  if (!doc.fileName.endsWith(".lp")) return;
  if (doc.languageId === "langp") return;
  try {
    await vscode.languages.setTextDocumentLanguage(doc, "langp");
    output.appendLine(`Set language to langp: ${doc.fileName}`);
  } catch (e) {
    output.appendLine(`Failed to set language: ${e}`);
  }
}

function parseDiagnostics(text: string, doc: vscode.TextDocument): vscode.Diagnostic[] {
  const diags: vscode.Diagnostic[] = [];
  const lines = text.split("\n");
  let i = 0;
  while (i < lines.length) {
    const line = lines[i];
    const errMatch = line.match(/^(error|warning)\[([^\]]+)\]:\s*(.+)$/i);
    if (errMatch) {
      const severity =
        errMatch[1].toLowerCase() === "warning"
          ? vscode.DiagnosticSeverity.Warning
          : vscode.DiagnosticSeverity.Error;
      let message = errMatch[3];
      // Include "help:" lines from lang check output
      while (i + 1 < lines.length && /^\s+help:/.test(lines[i + 1])) {
        i += 1;
        message += "\n" + lines[i].trim();
      }
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
      const end = docLine.text.length;
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

function runCheck(doc: vscode.TextDocument): void {
  if (doc.languageId !== "langp" && !doc.fileName.endsWith(".lp")) return;
  const lang = binPath("lang");
  if (!fs.existsSync(lang)) return;
  const tmp = path.join(os.tmpdir(), `langp-check-${Date.now()}.lp`);
  try {
    fs.writeFileSync(tmp, doc.getText());
    const result = cp.spawnSync(lang, ["check", tmp], { encoding: "utf8" });
    const combined = `${result.stdout}\n${result.stderr}`;
    const hasDiags = /^(error|warning)\[/m.test(combined);
    if (!hasDiags) {
      diagnosticCollection.set(doc.uri, []);
      return;
    }
    diagnosticCollection.set(doc.uri, parseDiagnostics(combined, doc));
  } catch (e) {
    output.appendLine(`check failed: ${e}`);
  } finally {
    try {
      fs.unlinkSync(tmp);
    } catch {
      /* ignore */
    }
  }
}

function scheduleCheck(doc: vscode.TextDocument): void {
  const cfg = vscode.workspace.getConfiguration("langp");
  if (!cfg.get<boolean>("checkOnType", true)) return;
  if (checkTimer) clearTimeout(checkTimer);
  checkTimer = setTimeout(() => runCheck(doc), 400);
}

function registerIntelliSense(context: vscode.ExtensionContext): void {
  const selector: vscode.DocumentSelector = { language: "langp", scheme: "file" };

  context.subscriptions.push(
    vscode.languages.registerCompletionItemProvider(
      selector,
      {
        provideCompletionItems(doc, position) {
          const prefix = wordPrefix(doc, position);
          const line = doc.lineAt(position.line).text;
          const before = line.slice(0, position.character);

          // After "input " suggest types (text, number, …)
          if (/\binput\s+$/.test(before) || /\binput\s+\w*$/.test(before)) {
            const typePrefix = before.match(/\binput\s+(\w*)$/)?.[1] ?? "";
            return INPUT_TYPES.filter((t) => matchesPrefix(t, typePrefix)).map((t) => {
              const item = new vscode.CompletionItem(t, vscode.CompletionItemKind.EnumMember);
              item.detail = `input ${t} "prompt"`;
              item.documentation = new vscode.MarkdownString(`Input type for \`input ${t}\``);
              item.insertText = new vscode.SnippetString(`${t} "\${1:prompt}"`);
              return item;
            });
          }

          const items: vscode.CompletionItem[] = [];
          const seen = new Set<string>();

          for (const v of variableCompletions(doc, prefix)) {
            if (!seen.has(v.label as string)) {
              seen.add(v.label as string);
              items.push(v);
            }
          }

          const addEntry = (entry: import("./langp-api").LangpEntry) => {
            if (!matchesPrefix(entry.name, prefix) || seen.has(entry.name)) return;
            seen.add(entry.name);
            items.push(entryToCompletion(entry, prefix));
          };

          for (const entry of STATEMENTS) addEntry(entry);
          for (const entry of BUILTINS) addEntry(entry);
          for (const entry of KEYWORDS) addEntry(entry);

          if (prefix.length >= 1) {
            for (const s of SNIPPETS) {
              const key = s.label;
              if (seen.has(key)) continue;
              if (
                matchesPrefix(s.label, prefix) ||
                matchesPrefix(s.label.replace(" block", ""), prefix)
              ) {
                seen.add(key);
                const item = new vscode.CompletionItem(s.label, vscode.CompletionItemKind.Snippet);
                item.detail = s.detail;
                item.insertText = new vscode.SnippetString(s.body);
                item.sortText = `2_${s.label}`;
                items.push(item);
              }
            }
          }

          return new vscode.CompletionList(items, false);
        },
      },
      ".",
      " ",
      "(",
      ",",
      "@"
    ),

    vscode.languages.registerSignatureHelpProvider(
      selector,
      {
        provideSignatureHelp(doc, position) {
          const line = doc.lineAt(position.line).text.slice(0, position.character);
          const fnMatch = line.match(/(\w+)\s*\(?[^()]*$/);
          if (!fnMatch) return null;
          const name = fnMatch[1];
          const entry = findEntry(name);
          if (!entry) return null;

          const sig = new vscode.SignatureInformation(
            entry.signature,
            new vscode.MarkdownString(entry.doc)
          );
          return {
            signatures: [sig],
            activeSignature: 0,
            activeParameter: 0,
          };
        },
      },
      "(",
      ",",
      " "
    ),

    vscode.languages.registerHoverProvider(selector, {
      provideHover(doc, position) {
        const prefix = wordPrefix(doc, position);
        if (!prefix) return null;
        const entry = findEntry(prefix);
        if (!entry) return null;
        return new vscode.Hover(
          new vscode.MarkdownString(`**${entry.name}**\n\n\`${entry.signature}\`\n\n${entry.doc}`)
        );
      },
    })
  );
}

function startLanguageServer(): void {
  const langpConfig = vscode.workspace.getConfiguration("langp");
  if (!langpConfig.get<boolean>("enableLanguageServer", true)) return;

  const configured = langpConfig.get<string>("languageServerPath", "").trim();
  const serverPath = configured || binPath("lang-lsp");
  if (!fs.existsSync(serverPath)) {
    output.appendLine("lang-lsp not found — using built-in IntelliSense");
    return;
  }

  const serverOptions: ServerOptions = {
    run: { command: serverPath, args: [], transport: TransportKind.stdio },
    debug: { command: serverPath, args: [], transport: TransportKind.stdio },
  };

  const clientOptions: LanguageClientOptions = {
    documentSelector: [{ scheme: "file", language: "langp", pattern: "**/*.lp" }],
    synchronize: { fileEvents: vscode.workspace.createFileSystemWatcher("**/*.lp") },
    outputChannel: output,
  };

  client = new LanguageClient("langp-lsp", "Lang.P LSP", serverOptions, clientOptions);
  void client.start().then(
    () => output.appendLine("lang-lsp connected"),
    (e) => output.appendLine(`lang-lsp failed: ${e}`)
  );
}

export async function activate(context: vscode.ExtensionContext): Promise<void> {
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

  context.subscriptions.push(
    diagnosticCollection,
    output,
    status,
    vscode.workspace.onDidOpenTextDocument(async (doc) => {
      await forceLangpLanguage(doc);
      runCheck(doc);
    }),
    vscode.workspace.onDidSaveTextDocument((doc) => runCheck(doc)),
    vscode.workspace.onDidChangeTextDocument((e) => {
      if (e.document.fileName.endsWith(".lp")) scheduleCheck(e.document);
    }),
    vscode.window.onDidChangeActiveTextEditor(async (editor) => {
      if (editor) {
        await forceLangpLanguage(editor.document);
        runCheck(editor.document);
      }
    }),
    vscode.commands.registerCommand("langp.runFile", () => {
      const editor = vscode.window.activeTextEditor;
      if (!editor?.document.fileName.endsWith(".lp")) return;
      const lang = binPath("lang");
      const term = vscode.window.createTerminal("Lang.P");
      term.show();
      term.sendText(`${lang} run "${editor.document.fileName}"`);
    }),
    vscode.commands.registerCommand("langp.checkFile", () => {
      const editor = vscode.window.activeTextEditor;
      if (editor) runCheck(editor.document);
    }),
    vscode.commands.registerCommand("langp.setLanguage", async () => {
      const editor = vscode.window.activeTextEditor;
      if (editor?.document.fileName.endsWith(".lp")) {
        await forceLangpLanguage(editor.document);
      }
    })
  );

  startLanguageServer();
}

export async function deactivate(): Promise<void> {
  if (checkTimer) clearTimeout(checkTimer);
  diagnosticCollection.dispose();
  if (client) await client.stop();
}
