//! Lang.P lexer — converts source text into a token stream.

use crate::error::{LexError, LexErrorKind, LexResult};
use crate::span::Span;
use crate::token::{InputTypeKeyword, Keyword, Token, TokenKind};

const INDENT_WIDTH: usize = 4;

/// Lexes Lang.P source code into tokens.
pub struct Lexer<'source> {
    source: &'source str,
    chars: std::iter::Peekable<std::str::Chars<'source>>,
    pos: usize,
    line: u32,
    column: u32,
    line_start: usize,
    at_line_start: bool,
    /// True after `handle_line_start` on this line; enables `..` block-close lexing.
    line_pending_block_close: bool,
    indent_stack: Vec<usize>,
    /// Nesting depth of `()`, `[]`, `{}` — indentation is ignored inside.
    delimiter_depth: u32,
    pending: Vec<Token>,
    errors: Vec<LexError>,
    /// When true, the token after `input` may be an input-type keyword.
    after_input: bool,
}

impl<'source> Lexer<'source> {
    pub fn new(source: &'source str) -> Self {
        Self {
            source,
            chars: source.chars().peekable(),
            pos: 0,
            line: 1,
            column: 1,
            line_start: 0,
            at_line_start: true,
            line_pending_block_close: false,
            indent_stack: vec![0],
            delimiter_depth: 0,
            pending: Vec::new(),
            errors: Vec::new(),
            after_input: false,
        }
    }

    /// Tokenize the entire source file.
    pub fn tokenize(mut self) -> LexResult<Vec<Token>> {
        let mut tokens = Vec::new();

        loop {
            let token = self.next_token()?;
            let is_eof = token.kind == TokenKind::Eof;
            tokens.push(token);
            if is_eof {
                break;
            }
        }

        if self.errors.is_empty() {
            Ok(tokens)
        } else {
            Err(self.errors.remove(0))
        }
    }

    /// Tokenize, collecting all errors (returns tokens even if errors occurred).
    pub fn tokenize_all(mut self) -> (Vec<Token>, Vec<LexError>) {
        let mut tokens = Vec::new();
        loop {
            match self.next_token() {
                Ok(token) => {
                    let is_eof = token.kind == TokenKind::Eof;
                    tokens.push(token);
                    if is_eof {
                        break;
                    }
                }
                Err(e) => {
                    self.errors.push(e);
                    if self.is_at_end() {
                        tokens.push(Token::new(
                            TokenKind::Eof,
                            self.span(self.pos, self.pos),
                            "",
                        ));
                        break;
                    }
                }
            }
        }
        (tokens, self.errors.clone())
    }

    fn next_token(&mut self) -> LexResult<Token> {
        if let Some(token) = self.pending.pop() {
            return Ok(token);
        }

        if self.at_line_start && !self.is_at_end() && !self.line_has_content() {
            let pos_before = self.pos;
            self.skip_blank_line();
            if self.pos != pos_before {
                return self.next_token();
            }
            self.at_line_start = false;
        }

        if self.at_line_start {
            self.handle_line_start()?;
            self.at_line_start = false;
            self.line_pending_block_close = true;
            if let Some(token) = self.pending.pop() {
                return Ok(token);
            }
        }

        if self.line_pending_block_close {
            if let Some(token) = self.try_lex_block_close()? {
                self.line_pending_block_close = false;
                return Ok(token);
            }
            self.line_pending_block_close = false;
        }

        self.skip_inline_whitespace();

        if self.is_at_end() {
            self.emit_eof_dedents();
            return Ok(self.make_token(TokenKind::Eof, self.pos, self.pos, ""));
        }

        self.at_line_start = false;

        let start = self.pos;
        let start_line = self.line;
        let start_col = self.column;

        let ch = self.peek_char().unwrap();

        // Comment
        if ch == '@' {
            self.skip_comment();
            return self.next_token();
        }

        // Newline
        if ch == '\n' {
            self.advance();
            self.at_line_start = true;
            return Ok(self.make_token(
                TokenKind::Newline,
                start,
                self.pos,
                "\\n",
            ));
        }

        if ch == '\r' {
            self.advance();
            if self.peek_char() == Some('\n') {
                self.advance();
            }
            self.at_line_start = true;
            return Ok(self.make_token(
                TokenKind::Newline,
                start,
                self.pos,
                "\\n",
            ));
        }

        // String literals
        if ch == '"' {
            if self.peek_str("\"\"\"") {
                return self.lex_raw_string(start, start_line, start_col);
            }
            return self.lex_string('"', start, start_line, start_col);
        }

        // Character literal or single-quoted string
        if ch == '\'' {
            return self.lex_char_or_string(start, start_line, start_col);
        }

        // Numbers
        if ch.is_ascii_digit() {
            return self.lex_number(start, start_line, start_col);
        }

        // Identifiers and keywords
        if is_ident_start(ch) {
            return self.lex_ident_or_keyword(start, start_line, start_col);
        }

        // Operators and punctuation
        self.lex_operator_or_punct(start, start_line, start_col)
    }

    fn handle_line_start(&mut self) -> LexResult<()> {
        let line_start_pos = self.pos;
        let mut indent = 0usize;

        while let Some(ch) = self.peek_char() {
            match ch {
                ' ' => {
                    indent += 1;
                    self.advance();
                }
                '\t' => {
                    return Err(LexError::new(
                        LexErrorKind::TabInIndent,
                        self.span(line_start_pos, self.pos + 1),
                        "tabs are not allowed for indentation; use 4 spaces",
                    ));
                }
                '@' => break,
                '\n' | '\r' => break,
                _ => break,
            }
        }

        if self.delimiter_depth > 0 {
            return Ok(());
        }

        if indent > 0 && indent % INDENT_WIDTH != 0 {
            return Err(LexError::new(
                LexErrorKind::IndentNotMultipleOfFour,
                self.span(line_start_pos, self.pos),
                format!(
                    "indentation must be a multiple of {} spaces, found {}",
                    INDENT_WIDTH, indent
                ),
            ));
        }

        let current = *self.indent_stack.last().unwrap();
        if indent > current {
            self.indent_stack.push(indent);
            self.pending.push(self.make_token(
                TokenKind::Indent,
                line_start_pos,
                self.pos,
                "INDENT",
            ));
        } else {
            while *self.indent_stack.last().unwrap() > indent {
                self.indent_stack.pop();
                self.pending.push(self.make_token(
                    TokenKind::Dedent,
                    line_start_pos,
                    self.pos,
                    "DEDENT",
                ));
            }
            if *self.indent_stack.last().unwrap() != indent {
                return Err(LexError::new(
                    LexErrorKind::InconsistentIndent,
                    self.span(line_start_pos, self.pos),
                    "inconsistent indentation",
                ));
            }
        }

        Ok(())
    }

    fn line_has_content(&self) -> bool {
        let mut idx = self.pos;
        let bytes = self.source.as_bytes();
        while idx < bytes.len() {
            let ch = bytes[idx] as char;
            match ch {
                ' ' | '\t' => idx += ch.len_utf8(),
                '@' | '\n' | '\r' => return false,
                _ => return true,
            }
        }
        false
    }

    fn skip_blank_line(&mut self) {
        while let Some(ch) = self.peek_char() {
            match ch {
                ' ' | '\t' => {
                    self.advance();
                }
                '@' => {
                    self.skip_comment();
                }
                '\n' => {
                    self.advance();
                    self.at_line_start = true;
                    break;
                }
                '\r' => {
                    self.advance();
                    if self.peek_char() == Some('\n') {
                        self.advance();
                    }
                    self.at_line_start = true;
                    break;
                }
                _ => break,
            }
        }
    }

    fn try_lex_block_close(&mut self) -> LexResult<Option<Token>> {
        if self.peek_str("..") {
            let next = self.source[self.pos + 2..].chars().next();
            if matches!(next, None | Some(' ' | '\t' | '\n' | '\r' | '@' | '.')) {
                let start = self.pos;
                self.advance();
                self.advance();
                return Ok(Some(self.make_token(TokenKind::BlockClose, start, self.pos, "..")));
            }
        }
        Ok(None)
    }

    fn emit_eof_dedents(&mut self) {
        while self.indent_stack.len() > 1 {
            self.indent_stack.pop();
            self.pending.push(self.make_token(
                TokenKind::Dedent,
                self.pos,
                self.pos,
                "DEDENT",
            ));
        }
    }

    fn lex_ident_or_keyword(
        &mut self,
        start: usize,
        line: u32,
        col: u32,
    ) -> LexResult<Token> {
        while let Some(ch) = self.peek_char() {
            if is_ident_continue(ch) {
                self.advance();
            } else {
                break;
            }
        }

        let text = &self.source[start..self.pos];

        // Compound keywords
        if text == "otherwise" && self.next_is_word("if") {
            self.skip_word("if");
            self.after_input = false;
            return Ok(Token::new(
                TokenKind::Keyword(Keyword::OtherwiseIf),
                Span::new(start, self.pos, line, col),
                "otherwise if",
            ));
        }
        if text == "repeat" && self.next_is_word("forever") {
            self.skip_word("forever");
            self.after_input = false;
            return Ok(Token::new(
                TokenKind::Keyword(Keyword::RepeatForever),
                Span::new(start, self.pos, line, col),
                "repeat forever",
            ));
        }
        if text == "wait" && self.next_is_word("for") {
            self.skip_word("for");
            self.after_input = false;
            return Ok(Token::new(
                TokenKind::Keyword(Keyword::WaitFor),
                Span::new(start, self.pos, line, col),
                "wait for",
            ));
        }

        if self.after_input {
            if let Some(input_ty) = InputTypeKeyword::from_ident(text) {
                self.after_input = false;
                return Ok(Token::new(
                    TokenKind::InputTypeKeyword(input_ty),
                    Span::new(start, self.pos, line, col),
                    text,
                ));
            }
        }

        if text == "input" {
            self.after_input = true;
        } else {
            self.after_input = false;
        }

        if let Some(kw) = Keyword::from_ident(text) {
            return Ok(Token::new(
                TokenKind::Keyword(kw),
                Span::new(start, self.pos, line, col),
                text,
            ));
        }

        Ok(Token::new(
            TokenKind::Ident(text.to_string()),
            Span::new(start, self.pos, line, col),
            text,
        ))
    }

    fn lex_number(&mut self, start: usize, line: u32, col: u32) -> LexResult<Token> {
        let first = self.peek_char().unwrap();

        // Hex, binary, octal
        if first == '0' {
            if self.peek_str("0x") || self.peek_str("0X") {
                self.advance();
                self.advance();
                return self.lex_digits_radix(16, start, line, col, |c| c.is_ascii_hexdigit());
            }
            if self.peek_str("0b") || self.peek_str("0B") {
                self.advance();
                self.advance();
                return self.lex_digits_radix(2, start, line, col, |c| c == '0' || c == '1');
            }
            if self.peek_str("0o") || self.peek_str("0O") {
                self.advance();
                self.advance();
                return self.lex_digits_radix(8, start, line, col, |c| matches!(c, '0'..='7'));
            }
        }

        while self.peek_char().is_some_and(|c| c.is_ascii_digit()) {
            self.advance();
        }

        // Float: must have digit after dot to distinguish from StmtEnd
        if self.peek_char() == Some('.') {
            if self.peek_next_is_digit() {
                self.advance(); // consume '.'
                while self.peek_char().is_some_and(|c| c.is_ascii_digit()) {
                    self.advance();
                }
                if matches!(self.peek_char(), Some('e' | 'E')) {
                    self.advance();
                    if matches!(self.peek_char(), Some('+' | '-')) {
                        self.advance();
                    }
                    if !self.peek_char().is_some_and(|c| c.is_ascii_digit()) {
                        return Err(LexError::new(
                            LexErrorKind::InvalidNumber,
                            Span::new(start, self.pos, line, col),
                            "invalid float exponent",
                        ));
                    }
                    while self.peek_char().is_some_and(|c| c.is_ascii_digit()) {
                        self.advance();
                    }
                }
                let text = &self.source[start..self.pos];
                let value: f64 = text.parse().map_err(|_| {
                    LexError::new(
                        LexErrorKind::InvalidNumber,
                        Span::new(start, self.pos, line, col),
                        format!("invalid float literal '{}'", text),
                    )
                })?;
                return Ok(Token::new(
                    TokenKind::Float(value),
                    Span::new(start, self.pos, line, col),
                    text,
                ));
            }
        }

        // Exponent-only float like 1e10
        if matches!(self.peek_char(), Some('e' | 'E')) {
            let before = self.pos;
            self.advance();
            if matches!(self.peek_char(), Some('+' | '-')) {
                self.advance();
            }
            if self.peek_char().is_some_and(|c| c.is_ascii_digit()) {
                while self.peek_char().is_some_and(|c| c.is_ascii_digit()) {
                    self.advance();
                }
                let text = &self.source[start..self.pos];
                let value: f64 = text.parse().map_err(|_| {
                    LexError::new(
                        LexErrorKind::InvalidNumber,
                        Span::new(start, self.pos, line, col),
                        format!("invalid float literal '{}'", text),
                    )
                })?;
                return Ok(Token::new(
                    TokenKind::Float(value),
                    Span::new(start, self.pos, line, col),
                    text,
                ));
            }
            // rewind — not a float exponent
            self.pos = before;
            self.sync_chars_to_pos();
        }

        let text = &self.source[start..self.pos];
        let value: i64 = text.parse().map_err(|_| {
            LexError::new(
                LexErrorKind::InvalidNumber,
                Span::new(start, self.pos, line, col),
                format!("invalid integer literal '{}'", text),
            )
        })?;
        Ok(Token::new(
            TokenKind::Int(value),
            Span::new(start, self.pos, line, col),
            text,
        ))
    }

    fn lex_digits_radix<F>(
        &mut self,
        radix: u32,
        start: usize,
        line: u32,
        col: u32,
        valid: F,
    ) -> LexResult<Token>
    where
        F: Fn(char) -> bool,
    {
        let digit_start = self.pos;
        while let Some(ch) = self.peek_char() {
            if valid(ch) {
                self.advance();
            } else {
                break;
            }
        }
        if self.pos == digit_start {
            return Err(LexError::new(
                LexErrorKind::InvalidNumber,
                Span::new(start, self.pos, line, col),
                "expected digits after radix prefix",
            ));
        }
        let text = &self.source[start..self.pos];
        let value = i64::from_str_radix(
            &text[2..],
            radix,
        )
        .map_err(|_| {
            LexError::new(
                LexErrorKind::InvalidNumber,
                Span::new(start, self.pos, line, col),
                format!("invalid integer literal '{}'", text),
            )
        })?;
        Ok(Token::new(
            TokenKind::Int(value),
            Span::new(start, self.pos, line, col),
            text,
        ))
    }

    fn lex_string(
        &mut self,
        quote: char,
        start: usize,
        line: u32,
        col: u32,
    ) -> LexResult<Token> {
        self.advance(); // opening quote
        let mut value = String::new();

        while let Some(ch) = self.peek_char() {
            if ch == quote {
                self.advance();
                return Ok(Token::new(
                    TokenKind::String(value),
                    Span::new(start, self.pos, line, col),
                    &self.source[start..self.pos],
                ));
            }
            if ch == '\n' || ch == '\r' {
                return Err(LexError::new(
                    LexErrorKind::UnterminatedString,
                    Span::new(start, self.pos, line, col),
                    "unterminated string literal",
                ));
            }
            if ch == '\\' {
                self.advance();
                value.push(self.decode_escape(line, col, start)?);
            } else {
                self.advance();
                value.push(ch);
            }
        }

        Err(LexError::new(
            LexErrorKind::UnterminatedString,
            Span::new(start, self.pos, line, col),
            "unterminated string literal",
        ))
    }

    fn lex_raw_string(&mut self, start: usize, line: u32, col: u32) -> LexResult<Token> {
        self.advance();
        self.advance();
        self.advance();
        let content_start = self.pos;

        while !self.is_at_end() {
            if self.peek_str("\"\"\"") {
                let end = self.pos;
                self.advance();
                self.advance();
                self.advance();
                let value = self.source[content_start..end].to_string();
                return Ok(Token::new(
                    TokenKind::String(value),
                    Span::new(start, self.pos, line, col),
                    &self.source[start..self.pos],
                ));
            }
            self.advance();
        }

        Err(LexError::new(
            LexErrorKind::UnterminatedRawString,
            Span::new(start, self.pos, line, col),
            "unterminated raw string literal",
        ))
    }

    fn lex_char_or_string(
        &mut self,
        start: usize,
        line: u32,
        col: u32,
    ) -> LexResult<Token> {
        self.advance();
        if self.peek_char() == Some('\'') {
            // empty '' — invalid
            return Err(LexError::new(
                LexErrorKind::UnterminatedCharacter,
                Span::new(start, self.pos, line, col),
                "empty character literal",
            ));
        }

        // Single-quoted string (multiple chars until closing ')
        let mut value = String::new();
        while let Some(ch) = self.peek_char() {
            if ch == '\'' {
                self.advance();
                if value.chars().count() == 1 {
                    return Ok(Token::new(
                        TokenKind::Char(value.chars().next().unwrap()),
                        Span::new(start, self.pos, line, col),
                        &self.source[start..self.pos],
                    ));
                }
                return Ok(Token::new(
                    TokenKind::String(value),
                    Span::new(start, self.pos, line, col),
                    &self.source[start..self.pos],
                ));
            }
            if ch == '\n' || ch == '\r' {
                return Err(LexError::new(
                    LexErrorKind::UnterminatedString,
                    Span::new(start, self.pos, line, col),
                    "unterminated string literal",
                ));
            }
            if ch == '\\' {
                self.advance();
                value.push(self.decode_escape(line, col, start)?);
            } else {
                self.advance();
                value.push(ch);
            }
        }

        Err(LexError::new(
            LexErrorKind::UnterminatedString,
            Span::new(start, self.pos, line, col),
            "unterminated string literal",
        ))
    }

    fn decode_escape(&mut self, line: u32, col: u32, start: usize) -> LexResult<char> {
        let ch = self.peek_char().ok_or_else(|| {
            LexError::new(
                LexErrorKind::InvalidEscape,
                Span::new(start, self.pos, line, col),
                "invalid escape sequence",
            )
        })?;
        match ch {
            'n' => {
                self.advance();
                Ok('\n')
            }
            't' => {
                self.advance();
                Ok('\t')
            }
            'r' => {
                self.advance();
                Ok('\r')
            }
            '\\' | '"' | '\'' => {
                self.advance();
                Ok(ch)
            }
            'u' => {
                self.advance();
                if self.peek_char() != Some('{') {
                    return Err(LexError::new(
                        LexErrorKind::InvalidUnicodeEscape,
                        Span::new(start, self.pos, line, col),
                        "expected '{' after \\u",
                    ));
                }
                self.advance();
                let hex_start = self.pos;
                while self.peek_char().is_some_and(|c| c.is_ascii_hexdigit()) {
                    self.advance();
                }
                if self.peek_char() != Some('}') {
                    return Err(LexError::new(
                        LexErrorKind::InvalidUnicodeEscape,
                        Span::new(start, self.pos, line, col),
                        "expected '}' to close unicode escape",
                    ));
                }
                self.advance();
                let hex = &self.source[hex_start..self.pos - 1];
                let code = u32::from_str_radix(hex, 16).map_err(|_| {
                    LexError::new(
                        LexErrorKind::InvalidUnicodeEscape,
                        Span::new(start, self.pos, line, col),
                        format!("invalid unicode code point '\\u{{{}}}'", hex),
                    )
                })?;
                char::from_u32(code).ok_or_else(|| {
                    LexError::new(
                        LexErrorKind::InvalidUnicodeEscape,
                        Span::new(start, self.pos, line, col),
                        format!("invalid unicode scalar value {}", code),
                    )
                })
            }
            _ => Err(LexError::new(
                LexErrorKind::InvalidEscape,
                Span::new(start, self.pos, line, col),
                format!("invalid escape sequence '\\{}'", ch),
            )),
        }
    }

    fn tok(&self, kind: TokenKind, start: usize, line: u32, col: u32, lexeme: &str) -> Token {
        Token::new(kind, Span::new(start, self.pos, line, col), lexeme)
    }

    fn emit_delimiter(
        &mut self,
        kind: TokenKind,
        start: usize,
        line: u32,
        col: u32,
        lexeme: &str,
    ) -> Token {
        match kind {
            TokenKind::LParen | TokenKind::LBracket | TokenKind::LBrace => {
                self.delimiter_depth += 1;
            }
            TokenKind::RParen | TokenKind::RBracket | TokenKind::RBrace => {
                self.delimiter_depth = self.delimiter_depth.saturating_sub(1);
            }
            _ => {}
        }
        self.tok(kind, start, line, col, lexeme)
    }

    fn lex_operator_or_punct(
        &mut self,
        start: usize,
        line: u32,
        col: u32,
    ) -> LexResult<Token> {
        let ch = self.advance().unwrap();

        match ch {
            '+' => {
                if self.peek_char() == Some('=') {
                    self.advance();
                    return Ok(self.tok(TokenKind::PlusEq, start, line, col, "+="));
                }
                Ok(self.tok(TokenKind::Plus, start, line, col, "+"))
            }
            '-' => {
                if self.peek_char() == Some('=') {
                    self.advance();
                    return Ok(self.tok(TokenKind::MinusEq, start, line, col, "-="));
                }
                if self.peek_char() == Some('>') {
                    self.advance();
                    return Ok(self.tok(TokenKind::Arrow, start, line, col, "->"));
                }
                Ok(self.tok(TokenKind::Minus, start, line, col, "-"))
            }
            '*' => {
                if self.peek_char() == Some('=') {
                    self.advance();
                    return Ok(self.tok(TokenKind::StarEq, start, line, col, "*="));
                }
                if self.peek_char() == Some('*') {
                    self.advance();
                    return Ok(self.tok(TokenKind::Pow, start, line, col, "**"));
                }
                Ok(self.tok(TokenKind::Star, start, line, col, "*"))
            }
            '/' => {
                if self.peek_char() == Some('=') {
                    self.advance();
                    return Ok(self.tok(TokenKind::SlashEq, start, line, col, "/="));
                }
                if self.peek_char() == Some('/') {
                    self.advance();
                    return Ok(self.tok(TokenKind::IntDiv, start, line, col, "//"));
                }
                Ok(self.tok(TokenKind::Slash, start, line, col, "/"))
            }
            '%' => {
                if self.peek_char() == Some('=') {
                    self.advance();
                    return Ok(self.tok(TokenKind::PercentEq, start, line, col, "%="));
                }
                Ok(self.tok(TokenKind::Percent, start, line, col, "%"))
            }
            '&' => {
                if self.peek_char() == Some('&') {
                    self.advance();
                    return Ok(self.tok(TokenKind::AndAnd, start, line, col, "&&"));
                }
                Ok(self.tok(TokenKind::Amp, start, line, col, "&"))
            }
            '|' => {
                if self.peek_char() == Some('|') {
                    self.advance();
                    return Ok(self.tok(TokenKind::OrOr, start, line, col, "||"));
                }
                Ok(self.tok(TokenKind::Pipe, start, line, col, "|"))
            }
            '^' => Ok(self.tok(TokenKind::Caret, start, line, col, "^")),
            '~' => Ok(self.tok(TokenKind::Tilde, start, line, col, "~")),
            '!' => {
                if self.peek_char() == Some('=') {
                    self.advance();
                    return Ok(self.tok(TokenKind::NotEq, start, line, col, "!="));
                }
                Ok(self.tok(TokenKind::Bang, start, line, col, "!"))
            }
            '=' => {
                if self.peek_char() == Some('=') {
                    self.advance();
                    return Ok(self.tok(TokenKind::EqEq, start, line, col, "=="));
                }
                if self.peek_char() == Some('>') {
                    self.advance();
                    return Ok(self.tok(TokenKind::FatArrow, start, line, col, "=>"));
                }
                Ok(self.tok(TokenKind::Eq, start, line, col, "="))
            }
            '<' => {
                if self.peek_str("..<") {
                    self.advance();
                    self.advance();
                    self.advance();
                    return Ok(self.tok(TokenKind::DotDotLt, start, line, col, "..<"));
                }
                if self.peek_str("<<") {
                    self.advance();
                    self.advance();
                    return Ok(self.tok(TokenKind::Shl, start, line, col, "<<"));
                }
                if self.peek_char() == Some('=') {
                    self.advance();
                    return Ok(self.tok(TokenKind::LtEq, start, line, col, "<="));
                }
                Ok(self.tok(TokenKind::Lt, start, line, col, "<"))
            }
            '>' => {
                if self.peek_str(">>") {
                    self.advance();
                    self.advance();
                    return Ok(self.tok(TokenKind::Shr, start, line, col, ">>"));
                }
                if self.peek_char() == Some('=') {
                    self.advance();
                    return Ok(self.tok(TokenKind::GtEq, start, line, col, ">="));
                }
                Ok(self.tok(TokenKind::Gt, start, line, col, ">"))
            }
            '?' => {
                if self.peek_char() == Some('?') {
                    self.advance();
                    return Ok(self.tok(TokenKind::NullCoalesce, start, line, col, "??"));
                }
                Ok(self.tok(TokenKind::Question, start, line, col, "?"))
            }
            '.' => self.lex_dot(start, line, col),
            ',' => {
                self.after_input = false;
                Ok(self.tok(TokenKind::Comma, start, line, col, ","))
            }
            ':' => Ok(self.tok(TokenKind::Colon, start, line, col, ":")),
            '(' => Ok(self.emit_delimiter(TokenKind::LParen, start, line, col, "(")),
            ')' => Ok(self.emit_delimiter(TokenKind::RParen, start, line, col, ")")),
            '[' => Ok(self.emit_delimiter(TokenKind::LBracket, start, line, col, "[")),
            ']' => Ok(self.emit_delimiter(TokenKind::RBracket, start, line, col, "]")),
            '{' => Ok(self.emit_delimiter(TokenKind::LBrace, start, line, col, "{")),
            '}' => Ok(self.emit_delimiter(TokenKind::RBrace, start, line, col, "}")),
            _ => Err(LexError::new(
                LexErrorKind::UnexpectedChar(ch),
                Span::new(start, self.pos, line, col),
                format!("unexpected character '{}'", ch),
            )),
        }
    }

    fn lex_dot(&mut self, start: usize, line: u32, col: u32) -> LexResult<Token> {
        // `...`
        if self.peek_str("...") {
            self.advance();
            self.advance();
            return Ok(Token::new(
                TokenKind::DotDotDot,
                Span::new(start, self.pos, line, col),
                "...",
            ));
        }
        // `..` (range — not block close; that is handled at line start)
        if self.peek_char() == Some('.') {
            self.advance();
            return Ok(Token::new(
                TokenKind::DotDot,
                Span::new(start, self.pos, line, col),
                "..",
            ));
        }

        // StmtEnd vs member access Dot
        if self.is_stmt_end_after_dot() {
            self.after_input = false;
            return Ok(Token::new(
                TokenKind::StmtEnd,
                Span::new(start, self.pos, line, col),
                ".",
            ));
        }

        Ok(Token::new(
            TokenKind::Dot,
            Span::new(start, self.pos, line, col),
            ".",
        ))
    }

    fn is_stmt_end_after_dot(&mut self) -> bool {
        let mut peek = self.source[self.pos..].chars().peekable();
        loop {
            match peek.next() {
                None => return true,
                Some(' ' | '\t') => continue,
                Some('\n' | '\r') => return true,
                Some('@') => {
                    // skip comment to end of line
                    for c in peek {
                        if c == '\n' || c == '\r' {
                            return true;
                        }
                    }
                    return true;
                }
                Some(',') => return true,
                Some(')') | Some(']') | Some('}') => return true,
                Some('.') => {
                    // `..` or `...` at expr level — not stmt end for single dot
                    // We already handled multi-dot above; a second `.` means range
                    return false;
                }
                Some(c) if c.is_ascii_alphanumeric() || c == '(' => return false,
                Some(_) => return true,
            }
        }
    }

    fn skip_comment(&mut self) {
        while let Some(ch) = self.peek_char() {
            if ch == '\n' || ch == '\r' {
                break;
            }
            self.advance();
        }
    }

    fn skip_inline_whitespace(&mut self) {
        while matches!(self.peek_char(), Some(' ' | '\t')) {
            self.advance();
        }
    }

    fn next_is_word(&self, word: &str) -> bool {
        let rest = &self.source[self.pos..];
        let trimmed = rest.trim_start();
        if !trimmed.starts_with(word) {
            return false;
        }
        match trimmed.chars().nth(word.len()) {
            None => true,
            Some(ch) => !is_ident_continue(ch),
        }
    }

    fn skip_word(&mut self, word: &str) {
        while matches!(self.peek_char(), Some(' ' | '\t')) {
            self.advance();
        }
        for _ in 0..word.len() {
            self.advance();
        }
    }

    fn peek_str(&self, s: &str) -> bool {
        self.source[self.pos..].starts_with(s)
    }

    fn peek_next_is_digit(&self) -> bool {
        self.source[self.pos..]
            .chars()
            .nth(1)
            .is_some_and(|c| c.is_ascii_digit())
    }

    fn peek_char(&self) -> Option<char> {
        self.source[self.pos..].chars().next()
    }

    fn advance(&mut self) -> Option<char> {
        let ch = self.chars.next()?;
        self.pos += ch.len_utf8();
        if ch == '\n' {
            self.line += 1;
            self.column = 1;
            self.line_start = self.pos;
        } else {
            self.column += 1;
        }
        Some(ch)
    }

    fn sync_chars_to_pos(&mut self) {
        self.chars = self.source[self.pos..].chars().peekable();
    }

    fn is_at_end(&self) -> bool {
        self.peek_char().is_none()
    }

    fn make_token(&self, kind: TokenKind, start: usize, end: usize, lexeme: &str) -> Token {
        Token::new(kind, self.span(start, end), lexeme)
    }

    fn span(&self, start: usize, end: usize) -> Span {
        // Approximate line/col from start — good enough for single-line tokens
        let (line, col) = self.offset_to_line_col(start);
        Span::new(start, end, line, col)
    }

    fn offset_to_line_col(&self, offset: usize) -> (u32, u32) {
        let mut line = 1u32;
        let mut col = 1u32;
        for (i, ch) in self.source.char_indices() {
            if i >= offset {
                break;
            }
            if ch == '\n' {
                line += 1;
                col = 1;
            } else {
                col += 1;
            }
        }
        (line, col)
    }
}

fn is_ident_start(ch: char) -> bool {
    ch.is_ascii_alphabetic() || ch == '_'
}

fn is_ident_continue(ch: char) -> bool {
    ch.is_ascii_alphanumeric() || ch == '_'
}

/// Convenience function to lex source text.
pub fn lex(source: &str) -> LexResult<Vec<Token>> {
    Lexer::new(source).tokenize()
}

/// Format tokens for debugging (`langc --emit tokens`).
pub fn format_tokens(tokens: &[Token]) -> String {
    tokens
        .iter()
        .map(|t| format!("{}", t.kind))
        .collect::<Vec<_>>()
        .join("\n")
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::token::TokenKind;

    fn kind_names(source: &str) -> Vec<String> {
        lex(source)
            .unwrap()
            .into_iter()
            .map(|t| t.kind.to_string())
            .collect()
    }

    #[test]
    fn hello_world() {
        let names = kind_names("print \"Hello\".");
        assert!(names.contains(&"print".to_string()));
        assert!(names.contains(&"STMT_END".to_string()));
        assert!(names.contains(&"EOF".to_string()));
    }

    #[test]
    fn member_vs_stmt_end() {
        let names = kind_names("user.name = \"Naga\".");
        assert_eq!(
            names,
            vec![
                "user", "DOT", "name", "=", "\"Naga\"", "STMT_END", "EOF"
            ]
        );
    }

    #[test]
    fn compound_keywords() {
        let tokens = lex("otherwise if x,\nrepeat forever,\nwait for x.").unwrap();
        assert!(tokens.iter().any(|t| t.kind == TokenKind::Keyword(Keyword::OtherwiseIf)));
        assert!(tokens.iter().any(|t| t.kind == TokenKind::Keyword(Keyword::RepeatForever)));
        assert!(tokens.iter().any(|t| t.kind == TokenKind::Keyword(Keyword::WaitFor)));
    }

    #[test]
    fn input_typed() {
        let tokens = lex("age = input number \"Age : \".").unwrap();
        assert!(tokens.iter().any(|t| t.kind == TokenKind::Keyword(Keyword::Input)));
        assert!(tokens.iter().any(|t| {
            t.kind == TokenKind::InputTypeKeyword(InputTypeKeyword::Number)
        }));
    }

    #[test]
    fn block_indent() {
        let tokens = lex("if true,\n    print \"yes\".\n..").unwrap();
        assert!(tokens.iter().any(|t| t.kind == TokenKind::Indent));
        assert!(tokens.iter().any(|t| t.kind == TokenKind::BlockClose));
    }

    #[test]
    fn integers_and_floats() {
        let tokens = lex("a = 42.\nb = 3.14.\nc = 0xFF.").unwrap();
        assert!(tokens.iter().any(|t| t.kind == TokenKind::Int(42)));
        assert!(tokens.iter().any(|t| t.kind == TokenKind::Float(3.14)));
        assert!(tokens.iter().any(|t| t.kind == TokenKind::Int(255)));
    }

    #[test]
    fn rejects_tabs_in_indent() {
        let err = lex("if true,\n\tprint x.\n..").unwrap_err();
        assert_eq!(err.kind, LexErrorKind::TabInIndent);
    }
}
