import * as vscode from "vscode";

const ASSIGN_RE = /^\s*([A-Za-z_]\w*)\s*=/gm;
const FUNCTION_PARAM_RE = /function\s+\w+\s*\(([^)]*)\)/g;
const REPEAT_AS_RE = /\brepeat\s+\S+\s+times\s+as\s+([A-Za-z_]\w*)/g;
const FOR_IN_RE = /\bfor\s+([A-Za-z_]\w*)\s+in\b/g;
const CATCH_RE = /\bcatch\s+([A-Za-z_]\w*)/g;

const RESERVED = new Set([
  "if", "otherwise", "else", "repeat", "forever", "for", "while", "try", "catch",
  "finally", "function", "type", "enum", "return", "break", "continue", "pass",
  "and", "or", "not", "in", "with", "as", "times", "true", "false", "null",
  "print", "input", "len", "to_string", "assert", "read", "read_bytes", "read_lines",
]);

function splitParams(raw: string): string[] {
  return raw
    .split(",")
    .map((p) => p.trim().split(/\s+/)[0]?.replace(/:.*/, "").trim())
    .filter((p) => /^[A-Za-z_]\w*$/.test(p));
}

/** Collect variable names visible in the document (assignments, params, loop vars). */
export function collectDocumentVariables(text: string): string[] {
  const found = new Set<string>();

  const add = (name: string | undefined) => {
    if (!name || RESERVED.has(name)) return;
    found.add(name);
  };

  for (const m of text.matchAll(ASSIGN_RE)) add(m[1]);
  for (const m of text.matchAll(FUNCTION_PARAM_RE)) {
    for (const p of splitParams(m[1])) add(p);
  }
  for (const m of text.matchAll(REPEAT_AS_RE)) add(m[1]);
  for (const m of text.matchAll(FOR_IN_RE)) add(m[1]);
  for (const m of text.matchAll(CATCH_RE)) add(m[1]);

  return [...found].sort();
}

export function variableCompletions(
  doc: vscode.TextDocument,
  prefix: string
): vscode.CompletionItem[] {
  return collectDocumentVariables(doc.getText())
    .filter((name) => !prefix || name.toLowerCase().startsWith(prefix.toLowerCase()))
    .map((name) => {
      const item = new vscode.CompletionItem(name, vscode.CompletionItemKind.Variable);
      item.detail = "variable";
      item.sortText = `00_${name}`;
      return item;
    });
}
