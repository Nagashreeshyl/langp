use langp_ast::{FunctionDecl, ModuleItem, Program};
use langp_parser::parse;
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::RwLock;
use tower_lsp::jsonrpc::Result;
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

    async fn publish_diagnostics(&self, _uri: &Url) {
        // Diagnostics are provided by the IDE extension via `lang check`
        // (single source of truth, friendly help lines, no stale LSP underlines).
    }

    fn analyze_document(text: &str) -> Option<Program> {
        parse(text).ok()
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
                completion_provider: None,
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

const KEYWORD_DOCS: &[(&str, &str)] = &[
    ("function", "Define a function. Body opens with `,` and closes with `..`"),
    ("if", "Conditional — use `otherwise if` / `otherwise` for else branches"),
    ("repeat", "`repeat N times,` or `repeat forever,`"),
    ("for", "`for item in collection,` loop"),
    ("while", "`while condition,` loop"),
    ("try", "`try,` … `catch name,` … `finally,` …"),
    ("input", "Read user input — `input text \"prompt\"` or typed variants"),
    ("with", "String concatenation in expressions: `\"Hello \" with name`"),
    ("print", "Print values — `print \"x\" with y.`"),
    ("len", "Length of string, list, or dict"),
    ("to_string", "Convert a value to string"),
    ("assert", "Fail if condition is false"),
    ("read", "Read file as text"),
    ("read_bytes", "Read file as bytes"),
    ("read_lines", "Read file as lines"),
    ("pass", "No-op statement"),
    ("null", "Null literal value"),
];
