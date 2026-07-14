import * as cp from "child_process";
import * as fs from "fs";
import * as os from "os";
import * as path from "path";
import * as vscode from "vscode";
import {
  CloseAction,
  ErrorAction,
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
      const lineText = docLine.text;
      // Underline the token at the error column, not the whole line (PyCharm-style).
      let start = Math.min(col, lineText.length);
      let end = start;
      if (start < lineText.length && /\S/.test(lineText[start])) {
        while (end < lineText.length && /\S/.test(lineText[end]) && lineText[end] !== ".") {
          end++;
        }
        if (end === start) end = Math.min(start + 1, lineText.length);
      } else {
        // Point error at end of line (e.g. missing `.`)
        start = Math.max(0, lineText.trimEnd().length);
        end = Math.min(start + 1, lineText.length);
      }
      diags.push({
        range: new vscode.Range(lineNum, start, lineNum, end),
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

/** Lang.P never uses `end` / `end.` — blocks close with `..` only. */
function forbiddenSyntaxDiags(doc: vscode.TextDocument): vscode.Diagnostic[] {
  const diags: vscode.Diagnostic[] = [];
  for (let i = 0; i < doc.lineCount; i++) {
    const line = doc.lineAt(i).text;
    const m = line.match(/^(\s*)end(\.)?\s*$/);
    if (!m) continue;
    const start = m[1].length;
    const endCol = start + 3 + (m[2] ? 1 : 0);
    diags.push({
      range: new vscode.Range(i, start, i, endCol),
      message:
        "Lang.P does not use `end`. Close blocks with `..` on its own line.\nhelp: replace `end.` with `..`",
      severity: vscode.DiagnosticSeverity.Error,
      source: "langp",
      code: "E0210",
    });
  }
  return diags;
}

function runCheck(doc: vscode.TextDocument): void {
  if (doc.languageId !== "langp" && !doc.fileName.endsWith(".lp")) return;
  const forbidden = forbiddenSyntaxDiags(doc);
  const lang = binPath("lang");
  if (!fs.existsSync(lang)) {
    diagnosticCollection.set(doc.uri, forbidden);
    return;
  }
  const tmp = path.join(os.tmpdir(), `langp-check-${Date.now()}.lp`);
  try {
    fs.writeFileSync(tmp, doc.getText());
    const result = cp.spawnSync(lang, ["check", tmp], { encoding: "utf8" });
    const combined = `${result.stdout}\n${result.stderr}`;
    const hasDiags = /^(error|warning)\[/m.test(combined);
    const langDiags = hasDiags ? parseDiagnostics(combined, doc) : [];
    diagnosticCollection.set(doc.uri, [...forbidden, ...langDiags]);
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
  if (!cfg.get<boolean>("checkOnType", false)) return;
  if (checkTimer) clearTimeout(checkTimer);
  checkTimer = setTimeout(() => runCheck(doc), 1200);
}

/** PyCharm-style: suggest while typing words, not after a finished statement. */
function shouldOfferCompletions(
  doc: vscode.TextDocument,
  position: vscode.Position
): boolean {
  const cfg = vscode.workspace.getConfiguration("langp");
  if (!cfg.get<boolean>("suggestWhileTyping", true)) return false;

  const line = doc.lineAt(position.line).text;
  const before = line.slice(0, position.character);
  const after = line.slice(position.character);

  // Finished statement or block — no popup
  if (/\.\s*$/.test(before) && after.trim() === "") return false;
  if (/^\s*\.\.\s*$/.test(before) && after.trim() === "") return false;

  // After "input " suggest types
  if (/\binput\s+\w*$/.test(before) || /\binput\s*$/.test(before)) return true;

  const prefix = wordPrefix(doc, position);
  return prefix.length >= 1;
}

function registerIntelliSense(context: vscode.ExtensionContext): void {
  const selector: vscode.DocumentSelector = { language: "langp", scheme: "file" };

  context.subscriptions.push(
    vscode.languages.registerCompletionItemProvider(
      selector,
      {
        provideCompletionItems(doc, position) {
          if (!shouldOfferCompletions(doc, position)) {
            return undefined;
          }

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
      }
      // No trigger characters — completions appear while typing words (quickSuggestions)
      // or on Ctrl+Space. Never pop up after typing `.` at end of a statement.
    ),

    vscode.languages.registerSignatureHelpProvider(
      selector,
      {
        provideSignatureHelp(doc, position) {
          const line = doc.lineAt(position.line).text.slice(0, position.character);
          const openIdx = line.lastIndexOf("(");
          if (openIdx === -1) return null;
          const closeIdx = line.indexOf(")", openIdx);
          if (closeIdx !== -1 && closeIdx >= position.character - 1) return null;

          const fnMatch = line.slice(0, openIdx).match(/(\w+)\s*$/);
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
      "("
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
    }),

    vscode.languages.registerCodeActionsProvider(selector, {
      provideCodeActions(doc, _range, context) {
        const actions: vscode.CodeAction[] = [];
        for (const diag of context.diagnostics) {
          if (diag.source !== "langp") continue;
          if (diag.code === "E0210") {
            const fix = new vscode.CodeAction(
              "Replace with .. (block close)",
              vscode.CodeActionKind.QuickFix
            );
            fix.diagnostics = [diag];
            fix.edit = new vscode.WorkspaceEdit();
            fix.edit.replace(doc.uri, diag.range, "..");
            actions.push(fix);
          }
          if (diag.code === "E0201" && diag.message.includes("not `.`")) {
            const fix = new vscode.CodeAction(
              "Replace . with .. (block close)",
              vscode.CodeActionKind.QuickFix
            );
            fix.diagnostics = [diag];
            fix.edit = new vscode.WorkspaceEdit();
            fix.edit.replace(doc.uri, diag.range, "..");
            actions.push(fix);
          }
        }
        return actions.length ? actions : undefined;
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
    middleware: {
      handleDiagnostics: () => {
        /* extension runCheck() is the single diagnostic source */
      },
    },
    errorHandler: {
      error: () => {
        output.appendLine("lang-lsp error — LSP disabled for this session");
        return { action: ErrorAction.Shutdown };
      },
      closed: () => ({ action: CloseAction.DoNotRestart }),
    },
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
    if (doc.fileName.endsWith(".lp")) {
      runCheck(doc);
    }
  }

  context.subscriptions.push(
    diagnosticCollection,
    output,
    status,
    vscode.workspace.onDidOpenTextDocument(async (doc) => {
      await forceLangpLanguage(doc);
      if (doc.fileName.endsWith(".lp")) {
        runCheck(doc);
      }
    }),
    vscode.workspace.onDidSaveTextDocument((doc) => {
      if (!doc.fileName.endsWith(".lp")) return;
      const cfg = vscode.workspace.getConfiguration("langp");
      if (cfg.get<boolean>("checkOnSave", true)) {
        runCheck(doc);
      }
    }),
    vscode.workspace.onDidChangeTextDocument((e) => {
      if (e.document.fileName.endsWith(".lp")) scheduleCheck(e.document);
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
