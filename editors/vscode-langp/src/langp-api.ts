import * as vscode from "vscode";
import * as manifest from "../langp-manifest.json";

export interface LangpEntry {
  name: string;
  kind: vscode.CompletionItemKind;
  signature: string;
  detail: string;
  doc: string;
  insertText?: string | vscode.SnippetString;
}

type ManifestItem = { name: string; signature: string; doc: string; snippet?: string };

function item(
  row: ManifestItem,
  kind: vscode.CompletionItemKind,
  detailLabel: string
): LangpEntry {
  return {
    name: row.name,
    kind,
    signature: row.signature,
    detail: detailLabel,
    doc: row.doc,
    insertText: row.snippet ? new vscode.SnippetString(row.snippet) : undefined,
  };
}

export const KEYWORDS: LangpEntry[] = manifest.keywords.map((k) =>
  item(k, vscode.CompletionItemKind.Keyword, "keyword")
);

export const STATEMENTS: LangpEntry[] = manifest.statements.map((s) =>
  item(s, vscode.CompletionItemKind.Keyword, "statement")
);

export const BUILTINS: LangpEntry[] = [
  ...manifest.builtins.map((b) => item(b, vscode.CompletionItemKind.Function, "builtin")),
  ...manifest.expressions.map((e) => item(e, vscode.CompletionItemKind.Function, "expression")),
];

export const INPUT_TYPES: string[] = manifest.inputTypes;

export const SNIPPETS = manifest.snippets;

export function wordPrefix(doc: vscode.TextDocument, position: vscode.Position): string {
  const line = doc.lineAt(position.line).text;
  let start = position.character;
  while (start > 0 && /[A-Za-z_]/.test(line[start - 1])) {
    start -= 1;
  }
  return line.slice(start, position.character);
}

export function matchesPrefix(name: string, prefix: string): boolean {
  if (!prefix) return true;
  return name.toLowerCase().startsWith(prefix.toLowerCase());
}

export function entryToCompletion(entry: LangpEntry, prefix: string): vscode.CompletionItem {
  const item = new vscode.CompletionItem(entry.name, entry.kind);
  item.detail = entry.signature;
  item.documentation = new vscode.MarkdownString(entry.doc);
  item.filterText = entry.name;
  item.sortText = entry.name.toLowerCase().startsWith(prefix.toLowerCase())
    ? `0_${entry.name}`
    : `1_${entry.name}`;
  if (entry.insertText !== undefined) {
    item.insertText = entry.insertText;
  }
  return item;
}

export function findEntry(name: string): LangpEntry | undefined {
  return (
    BUILTINS.find((b) => b.name === name) ??
    STATEMENTS.find((s) => s.name === name) ??
    KEYWORDS.find((k) => k.name === name)
  );
}

export function allCompletions(): LangpEntry[] {
  return [...STATEMENTS, ...BUILTINS, ...KEYWORDS];
}
