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

let client: LanguageClient | undefined;

const DEFAULT_LSP = path.join(os.homedir(), ".local", "bin", "lang-lsp");

const COLOR_RULES = [
  { scope: "comment.line.number-sign.langp", foreground: "#6A9955", fontStyle: "italic" },
  { scope: "string.quoted.double.langp", foreground: "#CE9178" },
  { scope: "keyword.control.langp", foreground: "#C586C0" },
  { scope: "keyword.control.compound.langp", foreground: "#C586C0" },
  { scope: "storage.type.function.langp", foreground: "#569CD6" },
  { scope: "storage.type.langp", foreground: "#569CD6" },
  { scope: "keyword.operator.word.langp", foreground: "#D4D4D4" },
  { scope: "constant.language.langp", foreground: "#569CD6" },
  { scope: "constant.numeric.integer.langp", foreground: "#B5CEA8" },
  { scope: "constant.numeric.float.langp", foreground: "#B5CEA8" },
  { scope: "support.function.langp", foreground: "#DCDCAA" },
  { scope: "entity.name.function.langp", foreground: "#DCDCAA" },
  { scope: "meta.function-call.langp", foreground: "#DCDCAA" },
  { scope: "entity.name.type.langp", foreground: "#4EC9B0" },
  { scope: "support.type.langp", foreground: "#4EC9B0" },
  { scope: "punctuation.section.block.end.langp", foreground: "#FFD700" },
  { scope: "punctuation.terminator.statement.langp", foreground: "#808080" },
  { scope: "variable.other.readwrite.langp", foreground: "#9CDCFE" },
  { scope: "keyword.operator.langp", foreground: "#D4D4D4" },
];

function resolveLangLsp(configured: string): string {
  if (configured.includes("/") || configured.includes("\\") || path.isAbsolute(configured)) {
    return configured;
  }
  const candidates = [
    DEFAULT_LSP,
    path.join(os.homedir(), ".cargo", "bin", configured),
  ];
  if (process.env.PATH) {
    for (const dir of process.env.PATH.split(path.delimiter)) {
      candidates.push(path.join(dir, configured));
    }
  }
  for (const candidate of candidates) {
    if (fs.existsSync(candidate)) return candidate;
  }
  return DEFAULT_LSP;
}

function applyLangpColors(): void {
  const editorConfig = vscode.workspace.getConfiguration("editor");
  const existing = editorConfig.get<Record<string, unknown>>("tokenColorCustomizations") ?? {};
  const rules = (existing.textMateRules as Array<Record<string, unknown>> | undefined) ?? [];
  const scopes = new Set(rules.map((r) => r.scope as string));
  const merged = [...rules];
  for (const rule of COLOR_RULES) {
    if (!scopes.has(rule.scope)) {
      merged.push({
        scope: rule.scope,
        settings: { foreground: rule.foreground, fontStyle: rule.fontStyle },
      });
    }
  }
  void editorConfig.update(
    "tokenColorCustomizations",
    { ...existing, textMateRules: merged },
    vscode.ConfigurationTarget.Global
  );
}

function applyEditorDefaults(): void {
  const config = vscode.workspace.getConfiguration();
  void config.update("files.associations", { "*.lp": "langp" }, vscode.ConfigurationTarget.Global);

  const langpEditor = vscode.workspace.getConfiguration("[langp]");
  const updates: [string, unknown][] = [
    ["editor.renderValidationDecorations", "on"],
    ["editor.showUnused", true],
    ["editor.glyphMargin", true],
  ];
  for (const [key, val] of updates) {
    if (langpEditor.get(key) === undefined) {
      void langpEditor.update(key, val, vscode.ConfigurationTarget.Global);
    }
  }
}

async function ensureLangpLanguage(doc: vscode.TextDocument): Promise<void> {
  if (doc.fileName.endsWith(".lp") && doc.languageId !== "langp") {
    await vscode.languages.setTextDocumentLanguage(doc, "langp");
  }
}

async function activateAllLpFiles(): Promise<void> {
  for (const doc of vscode.workspace.textDocuments) {
    await ensureLangpLanguage(doc);
  }
}

function startLanguageServer(context: vscode.ExtensionContext): void {
  const langpConfig = vscode.workspace.getConfiguration("langp");
  if (!langpConfig.get<boolean>("enableLanguageServer", true)) return;

  const raw = langpConfig.get<string>("languageServerPath", "").trim();
  const serverPath = resolveLangLsp(raw || "lang-lsp");

  if (!fs.existsSync(serverPath)) {
    void vscode.window.showWarningMessage(
      `Lang.P: lang-lsp not found at ${serverPath}. Run the install script.`
    );
    return;
  }

  const serverOptions: ServerOptions = {
    run: { command: serverPath, args: [], transport: TransportKind.stdio },
    debug: { command: serverPath, args: [], transport: TransportKind.stdio },
  };

  const clientOptions: LanguageClientOptions = {
    documentSelector: [
      { scheme: "file", language: "langp" },
      { scheme: "file", pattern: "**/*.lp" },
    ],
    synchronize: { fileEvents: vscode.workspace.createFileSystemWatcher("**/*.lp") },
    outputChannelName: "Lang.P Language Server",
  };

  client = new LanguageClient("langp", "Lang.P Language Server", serverOptions, clientOptions);

  void client.start();
}

export async function activate(context: vscode.ExtensionContext): Promise<void> {
  applyLangpColors();
  applyEditorDefaults();
  await activateAllLpFiles();

  context.subscriptions.push(
    vscode.workspace.onDidOpenTextDocument((doc) => void ensureLangpLanguage(doc)),
    vscode.window.onDidChangeActiveTextEditor((editor) => {
      if (editor) void ensureLangpLanguage(editor.document);
    })
  );

  startLanguageServer(context);
}

export async function deactivate(): Promise<void> {
  if (client) {
    await client.stop();
    client = undefined;
  }
}
