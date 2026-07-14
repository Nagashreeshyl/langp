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
exports.INPUT_TYPES = exports.BUILTINS = exports.KEYWORDS = void 0;
exports.wordPrefix = wordPrefix;
exports.matchesPrefix = matchesPrefix;
exports.entryToCompletion = entryToCompletion;
exports.findEntry = findEntry;
const vscode = __importStar(require("vscode"));
exports.KEYWORDS = [
    { name: "if", kind: vscode.CompletionItemKind.Keyword, signature: "if condition,", detail: "keyword", doc: "Conditional block — closes with `..`" },
    { name: "otherwise", kind: vscode.CompletionItemKind.Keyword, signature: "otherwise,", detail: "keyword", doc: "Else branch for `if`" },
    { name: "otherwise if", kind: vscode.CompletionItemKind.Keyword, signature: "otherwise if condition,", detail: "keyword", doc: "Else-if branch" },
    { name: "else", kind: vscode.CompletionItemKind.Keyword, signature: "else,", detail: "keyword", doc: "Alternative branch" },
    { name: "repeat", kind: vscode.CompletionItemKind.Keyword, signature: "repeat N times,", detail: "keyword", doc: "Repeat a block N times" },
    { name: "repeat forever", kind: vscode.CompletionItemKind.Keyword, signature: "repeat forever,", detail: "keyword", doc: "Infinite loop" },
    { name: "for", kind: vscode.CompletionItemKind.Keyword, signature: "for item in collection,", detail: "keyword", doc: "Iterate over a collection" },
    { name: "while", kind: vscode.CompletionItemKind.Keyword, signature: "while condition,", detail: "keyword", doc: "Loop while condition is true" },
    { name: "try", kind: vscode.CompletionItemKind.Keyword, signature: "try,", detail: "keyword", doc: "Try/catch error handling" },
    { name: "catch", kind: vscode.CompletionItemKind.Keyword, signature: "catch error,", detail: "keyword", doc: "Handle errors from try block" },
    { name: "finally", kind: vscode.CompletionItemKind.Keyword, signature: "finally,", detail: "keyword", doc: "Always runs after try/catch" },
    { name: "function", kind: vscode.CompletionItemKind.Keyword, signature: "function name(params),", detail: "keyword", doc: "Define a function" },
    { name: "type", kind: vscode.CompletionItemKind.Keyword, signature: "type Name,", detail: "keyword", doc: "Define a record type" },
    { name: "enum", kind: vscode.CompletionItemKind.Keyword, signature: "enum Name,", detail: "keyword", doc: "Define an enumeration" },
    { name: "use", kind: vscode.CompletionItemKind.Keyword, signature: "use module.", detail: "keyword", doc: "Import a module" },
    { name: "let", kind: vscode.CompletionItemKind.Keyword, signature: "let name = value.", detail: "keyword", doc: "Bind a variable" },
    { name: "return", kind: vscode.CompletionItemKind.Keyword, signature: "return value.", detail: "keyword", doc: "Return from function" },
    { name: "break", kind: vscode.CompletionItemKind.Keyword, signature: "break.", detail: "keyword", doc: "Exit loop" },
    { name: "continue", kind: vscode.CompletionItemKind.Keyword, signature: "continue.", detail: "keyword", doc: "Next loop iteration" },
    { name: "and", kind: vscode.CompletionItemKind.Keyword, signature: "and", detail: "keyword", doc: "Logical AND" },
    { name: "or", kind: vscode.CompletionItemKind.Keyword, signature: "or", detail: "keyword", doc: "Logical OR" },
    { name: "not", kind: vscode.CompletionItemKind.Keyword, signature: "not", detail: "keyword", doc: "Logical NOT" },
    { name: "in", kind: vscode.CompletionItemKind.Keyword, signature: "in", detail: "keyword", doc: "Membership / for-loop" },
    { name: "as", kind: vscode.CompletionItemKind.Keyword, signature: "as", detail: "keyword", doc: "Alias in loops" },
    { name: "with", kind: vscode.CompletionItemKind.Keyword, signature: "with", detail: "keyword", doc: "String concatenation in expressions" },
    { name: "times", kind: vscode.CompletionItemKind.Keyword, signature: "times", detail: "keyword", doc: "Used with `repeat N times`" },
    { name: "true", kind: vscode.CompletionItemKind.Keyword, signature: "true", detail: "keyword", doc: "Boolean true" },
    { name: "false", kind: vscode.CompletionItemKind.Keyword, signature: "false", detail: "keyword", doc: "Boolean false" },
    { name: "null", kind: vscode.CompletionItemKind.Keyword, signature: "null", detail: "keyword", doc: "Null value" },
    { name: "pass", kind: vscode.CompletionItemKind.Keyword, signature: "pass.", detail: "keyword", doc: "No-op statement" },
];
exports.BUILTINS = [
    {
        name: "print",
        kind: vscode.CompletionItemKind.Function,
        signature: 'print "message" with value.',
        detail: "print(...)",
        doc: "Print to stdout. Chain parts with `with`.",
        insertText: 'print "${1:message}".',
    },
    {
        name: "input",
        kind: vscode.CompletionItemKind.Function,
        signature: 'input text "prompt"',
        detail: "input(type, prompt)",
        doc: "Read user input. Types: `text`, `number`, `decimal`, `boolean`, `password`.",
        insertText: 'input text "${1:prompt}"',
    },
    {
        name: "len",
        kind: vscode.CompletionItemKind.Function,
        signature: "len(value)",
        detail: "len(value) → Int",
        doc: "Length of string or list.",
        insertText: "len(${1:value})",
    },
    {
        name: "range",
        kind: vscode.CompletionItemKind.Function,
        signature: "range(n)",
        detail: "range(n) → List",
        doc: "Create a list of integers 0..n-1.",
        insertText: "range(${1:5})",
    },
    {
        name: "str",
        kind: vscode.CompletionItemKind.Function,
        signature: "str(value)",
        detail: "str(value) → String",
        doc: "Convert value to string.",
        insertText: "str(${1:value})",
    },
    {
        name: "int",
        kind: vscode.CompletionItemKind.Function,
        signature: "int(value)",
        detail: "int(value) → Int",
        doc: "Convert value to integer.",
        insertText: "int(${1:value})",
    },
    {
        name: "assert",
        kind: vscode.CompletionItemKind.Function,
        signature: "assert condition.",
        detail: "assert(condition)",
        doc: "Fail if condition is false.",
        insertText: "assert ${1:condition}.",
    },
];
exports.INPUT_TYPES = ["text", "number", "decimal", "boolean", "password"];
function wordPrefix(doc, position) {
    const line = doc.lineAt(position.line).text;
    let start = position.character;
    while (start > 0 && /[A-Za-z_]/.test(line[start - 1])) {
        start -= 1;
    }
    return line.slice(start, position.character);
}
function matchesPrefix(name, prefix) {
    if (!prefix)
        return true;
    return name.toLowerCase().startsWith(prefix.toLowerCase());
}
function entryToCompletion(entry, prefix) {
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
function findEntry(name) {
    return exports.BUILTINS.find((b) => b.name === name) ?? exports.KEYWORDS.find((k) => k.name === name);
}
//# sourceMappingURL=langp-api.js.map