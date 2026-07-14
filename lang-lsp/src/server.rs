use langp_ast::{FunctionDecl, ModuleItem, Program};
use langp_parser::parse;
use langp_semantic::{analyze, Diagnostic as SemanticDiagnostic, Severity};
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::RwLock;
use tower_lsp::jsonrpc::Result;
use tower_lsp::lsp_types::Diagnostic as LspDiagnostic;
use tower_lsp::lsp_types::*;
use tower_lsp::Client;

type Documents = Arc<RwLock<HashMap<String, String>>>;

pub struct LangpServer {
    client: Client,
    documents: Documents,
}

impl LangpServer {
    pub fn new(client: Client) -> Self {
        Self {
            client,
            documents: Arc::new(RwLock::new(HashMap::new())),
        }
    }

    async fn publish_diagnostics(&self, uri: &Url) {
        let docs = self.documents.read().await;
        let Some(text) = docs.get(uri.as_str()) else {
            return;
        };

        let mut diagnostics = Vec::new();

        match parse(text) {
            Ok(program) => {
                let result = analyze(&program);
                for d in result.diagnostics {
                    diagnostics.push(to_lsp_diagnostic(&d));
                }
            }
            Err(e) => {
                diagnostics.push(LspDiagnostic {
                    range: span_to_range(text, e.span.start, e.span.end),
                    severity: Some(DiagnosticSeverity::ERROR),
                    code: Some(NumberOrString::String("E0200".into())),
                    source: Some("langp".into()),
                    message: e.message,
                    ..Default::default()
                });
            }
        }

        self.client
            .publish_diagnostics(uri.clone(), diagnostics, None)
            .await;
    }

    fn analyze_document(text: &str) -> Option<Program> {
        parse(text).ok()
    }

    fn symbols_from_program(program: &Program) -> Vec<(String, CompletionItemKind)> {
        let mut out = Vec::new();
        for item in &program.items {
            match item {
                ModuleItem::Function(f) => {
                    out.push((f.name.clone(), CompletionItemKind::FUNCTION));
                }
                ModuleItem::Type(t) => {
                    out.push((t.name.clone(), CompletionItemKind::CLASS));
                }
                ModuleItem::Enum(e) => {
                    out.push((e.name.clone(), CompletionItemKind::ENUM));
                }
                _ => {}
            }
        }
        out
    }

    fn functions_from_program(program: &Program) -> Vec<&FunctionDecl> {
        program
            .items
            .iter()
            .filter_map(|i| match i {
                ModuleItem::Function(f) => Some(f),
                _ => None,
            })
            .collect()
    }
}

#[tower_lsp::async_trait]
impl tower_lsp::LanguageServer for LangpServer {
    async fn initialize(&self, _: InitializeParams) -> Result<InitializeResult> {
        Ok(InitializeResult {
            server_info: Some(ServerInfo {
                name: "lang-lsp".into(),
                version: Some(env!("CARGO_PKG_VERSION").into()),
            }),
            capabilities: ServerCapabilities {
                text_document_sync: Some(TextDocumentSyncCapability::Kind(
                    TextDocumentSyncKind::FULL,
                )),
                completion_provider: Some(CompletionOptions {
                    trigger_characters: Some(vec![
                        ".".into(),
                        " ".into(),
                        "(".into(),
                        ",".into(),
                    ]),
                    resolve_provider: Some(false),
                    ..Default::default()
                }),
                hover_provider: Some(HoverProviderCapability::Simple(true)),
                definition_provider: Some(OneOf::Left(true)),
                document_symbol_provider: Some(OneOf::Left(true)),
                ..Default::default()
            },
            ..Default::default()
        })
    }

    async fn initialized(&self, _: InitializedParams) {
        self.client
            .log_message(MessageType::INFO, "Lang.P language server ready")
            .await;
    }

    async fn shutdown(&self) -> Result<()> {
        Ok(())
    }

    async fn did_open(&self, params: DidOpenTextDocumentParams) {
        let uri = params.text_document.uri;
        let text = params.text_document.text;
        self.documents
            .write()
            .await
            .insert(uri.to_string(), text);
        self.publish_diagnostics(&uri).await;
    }

    async fn did_change(&self, params: DidChangeTextDocumentParams) {
        let uri = params.text_document.uri;
        if let Some(change) = params.content_changes.into_iter().last() {
            self.documents
                .write()
                .await
                .insert(uri.to_string(), change.text);
        }
        self.publish_diagnostics(&uri).await;
    }

    async fn did_save(&self, params: DidSaveTextDocumentParams) {
        self.publish_diagnostics(&params.text_document.uri).await;
    }

    async fn completion(&self, params: CompletionParams) -> Result<Option<CompletionResponse>> {
        let uri = params.text_document_position.text_document.uri.to_string();
        let docs = self.documents.read().await;
        let Some(text) = docs.get(&uri) else {
            return Ok(None);
        };

        let mut items = keyword_completions();

        if let Some(program) = Self::analyze_document(text) {
            for (name, kind) in Self::symbols_from_program(&program) {
                items.push(CompletionItem {
                    label: name.clone(),
                    kind: Some(kind),
                    detail: Some("Lang.P symbol".into()),
                    insert_text: Some(name),
                    ..Default::default()
                });
            }
        }

        for builtin in BUILTINS {
            items.push(CompletionItem {
                label: (*builtin).into(),
                kind: Some(CompletionItemKind::FUNCTION),
                detail: Some("builtin".into()),
                ..Default::default()
            });
        }

        for snippet in SNIPPETS {
            items.push(CompletionItem {
                label: snippet.label.into(),
                kind: Some(CompletionItemKind::SNIPPET),
                detail: Some(snippet.detail.into()),
                insert_text: Some(snippet.body.into()),
                insert_text_format: Some(InsertTextFormat::SNIPPET),
                ..Default::default()
            });
        }

        Ok(Some(CompletionResponse::Array(items)))
    }

    async fn hover(&self, params: HoverParams) -> Result<Option<Hover>> {
        let uri = params.text_document_position_params.text_document.uri.to_string();
        let pos = params.text_document_position_params.position;
        let docs = self.documents.read().await;
        let Some(text) = docs.get(&uri) else {
            return Ok(None);
        };

        let word = word_at_position(text, pos.line as usize, pos.character as usize);
        let Some(word) = word else {
            return Ok(None);
        };

        if let Some((_, doc)) = KEYWORD_DOCS.iter().find(|(k, _)| *k == word.as_str()) {
            return Ok(Some(Hover {
                contents: HoverContents::Markup(MarkupContent {
                    kind: MarkupKind::Markdown,
                    value: (*doc).into(),
                }),
                range: None,
            }));
        }

        if let Some(program) = Self::analyze_document(text) {
            for f in Self::functions_from_program(&program) {
                if f.name == word {
                    let params: Vec<_> = f.params.iter().map(|p| p.name.as_str()).collect();
                    let sig = format!("function {}({})", f.name, params.join(", "));
                    return Ok(Some(Hover {
                        contents: HoverContents::Markup(MarkupContent {
                            kind: MarkupKind::Markdown,
                            value: format!("```langp\n{sig}\n```"),
                        }),
                        range: None,
                    }));
                }
            }
        }

        Ok(None)
    }

    async fn goto_definition(
        &self,
        params: GotoDefinitionParams,
    ) -> Result<Option<GotoDefinitionResponse>> {
        let uri = params.text_document_position_params.text_document.uri.to_string();
        let pos = params.text_document_position_params.position;
        let docs = self.documents.read().await;
        let Some(text) = docs.get(&uri) else {
            return Ok(None);
        };

        let Some(word) = word_at_position(text, pos.line as usize, pos.character as usize) else {
            return Ok(None);
        };
        let Some(program) = Self::analyze_document(text) else {
            return Ok(None);
        };

        for item in &program.items {
            let span = match item {
                ModuleItem::Function(f) if f.name == word => Some(f.span),
                ModuleItem::Type(t) if t.name == word => Some(t.span),
                ModuleItem::Enum(e) if e.name == word => Some(e.span),
                _ => None,
            };
            if let Some(s) = span {
                return Ok(Some(GotoDefinitionResponse::Scalar(Location {
                    uri: params
                        .text_document_position_params
                        .text_document
                        .uri
                        .clone(),
                    range: span_to_range(text, s.start, s.end),
                })));
            }
        }

        Ok(None)
    }

    async fn document_symbol(
        &self,
        params: DocumentSymbolParams,
    ) -> Result<Option<DocumentSymbolResponse>> {
        let uri = params.text_document.uri.to_string();
        let docs = self.documents.read().await;
        let Some(text) = docs.get(&uri) else {
            return Ok(None);
        };
        let Some(program) = Self::analyze_document(text) else {
            return Ok(None);
        };

        let mut symbols = Vec::new();
        for item in &program.items {
            match item {
                ModuleItem::Function(f) => {
                    symbols.push(DocumentSymbol {
                        name: f.name.clone(),
                        detail: Some("function".into()),
                        kind: SymbolKind::FUNCTION,
                        range: span_to_range(text, f.span.start, f.span.end),
                        selection_range: span_to_range(text, f.span.start, f.span.end),
                        children: None,
                        tags: None,
                        deprecated: None,
                    });
                }
                ModuleItem::Type(t) => {
                    symbols.push(DocumentSymbol {
                        name: t.name.clone(),
                        detail: Some("type".into()),
                        kind: SymbolKind::CLASS,
                        range: span_to_range(text, t.span.start, t.span.end),
                        selection_range: span_to_range(text, t.span.start, t.span.end),
                        children: None,
                        tags: None,
                        deprecated: None,
                    });
                }
                ModuleItem::Enum(e) => {
                    symbols.push(DocumentSymbol {
                        name: e.name.clone(),
                        detail: Some("enum".into()),
                        kind: SymbolKind::ENUM,
                        range: span_to_range(text, e.span.start, e.span.end),
                        selection_range: span_to_range(text, e.span.start, e.span.end),
                        children: None,
                        tags: None,
                        deprecated: None,
                    });
                }
                _ => {}
            }
        }

        Ok(Some(DocumentSymbolResponse::Nested(symbols)))
    }
}

fn to_lsp_diagnostic(d: &SemanticDiagnostic) -> LspDiagnostic {
    LspDiagnostic {
        range: Range {
            start: Position {
                line: d.span.line.saturating_sub(1),
                character: d.span.column.saturating_sub(1),
            },
            end: Position {
                line: d.span.line.saturating_sub(1),
                character: d.span.column.saturating_sub(1) + 1,
            },
        },
        severity: Some(match d.severity {
            Severity::Error => DiagnosticSeverity::ERROR,
            Severity::Warning => DiagnosticSeverity::WARNING,
            Severity::Info => DiagnosticSeverity::INFORMATION,
        }),
        code: Some(NumberOrString::String(d.code().into())),
        source: Some("langp".into()),
        message: d.message.clone(),
        ..Default::default()
    }
}

fn span_to_range(source: &str, start: usize, end: usize) -> Range {
    let start_pos = offset_to_position(source, start);
    let end_pos = offset_to_position(source, end);
    Range {
        start: start_pos,
        end: end_pos,
    }
}

fn offset_to_position(source: &str, offset: usize) -> Position {
    let mut line = 0u32;
    let mut col = 0u32;
    for (i, ch) in source.chars().enumerate() {
        if i >= offset {
            break;
        }
        if ch == '\n' {
            line += 1;
            col = 0;
        } else {
            col += 1;
        }
    }
    Position {
        line,
        character: col,
    }
}

fn word_at_position(source: &str, line: usize, character: usize) -> Option<String> {
    let line_text = source.lines().nth(line)?;
    let chars: Vec<char> = line_text.chars().collect();
    if character >= chars.len() {
        return None;
    }
    let mut start = character;
    let mut end = character;
    while start > 0 && is_ident_part(chars[start - 1]) {
        start -= 1;
    }
    while end < chars.len() && is_ident_part(chars[end]) {
        end += 1;
    }
    if start == end {
        return None;
    }
    Some(chars[start..end].iter().collect())
}

fn is_ident_part(c: char) -> bool {
    c.is_ascii_alphanumeric() || c == '_'
}

fn keyword_completions() -> Vec<CompletionItem> {
    KEYWORDS
        .iter()
        .map(|kw| CompletionItem {
            label: (*kw).into(),
            kind: Some(CompletionItemKind::KEYWORD),
            detail: Some("keyword".into()),
            ..Default::default()
        })
        .collect()
}

const KEYWORDS: &[&str] = &[
    "and", "as", "async", "await", "break", "catch", "continue", "else", "enum", "false",
    "finally", "for", "forever", "function", "if", "in", "input", "interface", "let", "match",
    "not", "null", "on", "or", "otherwise", "otherwise if", "repeat", "repeat forever", "return",
    "self", "static", "super", "this", "true", "try", "type", "use", "wait", "wait for", "while",
    "with",
];

const BUILTINS: &[&str] = &[
    "print", "len", "to_string", "assert", "read", "read_bytes", "read_lines", "write",
    "write_bytes", "append", "get", "post", "put", "delete", "patch", "copy", "move", "rename",
    "pass",
];

struct SnippetDef {
    label: &'static str,
    detail: &'static str,
    body: &'static str,
}

const SNIPPETS: &[SnippetDef] = &[
    SnippetDef {
        label: "function",
        detail: "function declaration",
        body: "function ${1:name}(${2:params}),\n    ${3:pass}.\n.",
    },
    SnippetDef {
        label: "if",
        detail: "if statement",
        body: "if ${1:condition},\n    ${2:pass}\n..",
    },
    SnippetDef {
        label: "type",
        detail: "type declaration",
        body: "type ${1:Name},\n    ${2:field}: ${3:String}.\n.",
    },
    SnippetDef {
        label: "input",
        detail: "input expression",
        body: "input ${1|text,number,decimal,boolean,password|} \"${2:prompt}\"",
    },
];

const KEYWORD_DOCS: &[(&str, &str)] = &[
    ("function", "Define a function. Body opens with `,` and closes with `.`"),
    ("type", "Define a record/class type with fields and methods"),
    ("enum", "Define an enumeration type"),
    ("if", "Conditional — use `otherwise if` / `otherwise` for else branches"),
    ("repeat", "`repeat N times,` or `repeat forever,`"),
    ("for", "`for item in collection,` loop"),
    ("while", "`while condition,` loop"),
    ("try", "`try,` … `catch name,` … `finally,` …"),
    ("input", "Read user input — `input text \"prompt\"` or typed variants"),
    ("with", "String concatenation in expressions: `\"Hello \" with name`"),
    ("print", "Print values — `print \"x\" with y.`"),
    ("use", "Import a module — `use module.name.`"),
    ("on", "Event handler — `on event,` … `..`"),
    ("await", "Used with `wait for` for async HTTP"),
    ("null", "Null literal value"),
    ("self", "Reference to current object in methods"),
];
