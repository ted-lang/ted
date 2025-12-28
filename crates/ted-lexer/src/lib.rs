//! Lexer for the Ted language.
//!
//! Transforms source text into a stream of tokens.

use ted_diagnostics::Diagnostics;
use ted_ir::{SourceId, Span};

/// The kind of a token.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TokenKind {
    // Literals
    /// Integer literal (e.g., `42`, `0xFF`)
    Integer,
    /// String literal (e.g., `"hello"`)
    String,
    /// Identifier (e.g., `foo`, `bar_baz`)
    Ident,

    // Keywords (placeholders - extend as language is defined)
    /// `module` keyword
    Module,
    /// `signal` keyword
    Signal,
    /// `clock` keyword
    Clock,
    /// `delay` keyword
    Delay,
    /// `on` keyword
    On,

    // Punctuation
    /// `(`
    LParen,
    /// `)`
    RParen,
    /// `{`
    LBrace,
    /// `}`
    RBrace,
    /// `[`
    LBracket,
    /// `]`
    RBracket,
    /// `;`
    Semi,
    /// `:`
    Colon,
    /// `,`
    Comma,
    /// `=`
    Eq,
    /// `->`
    Arrow,

    // Special
    /// End of file
    Eof,
    /// Unknown/invalid character
    Unknown,
}

/// A token from the source code.
#[derive(Debug, Clone)]
pub struct Token {
    /// The kind of token.
    pub kind: TokenKind,
    /// The span in source.
    pub span: Span,
    /// The raw text of the token.
    pub text: String,
}

impl Token {
    /// Creates a new token.
    #[must_use]
    pub fn new(kind: TokenKind, span: Span, text: impl Into<String>) -> Self {
        Self {
            kind,
            span,
            text: text.into(),
        }
    }
}

/// The lexer for Ted source code.
pub struct Lexer<'src> {
    source_id: SourceId,
    source: &'src str,
    pos: usize,
    diagnostics: Diagnostics,
}

impl<'src> Lexer<'src> {
    /// Creates a new lexer for the given source.
    #[must_use]
    pub fn new(source_id: SourceId, source: &'src str) -> Self {
        Self {
            source_id,
            source,
            pos: 0,
            diagnostics: Diagnostics::new(),
        }
    }

    /// Tokenizes the entire source, returning all tokens.
    pub fn tokenize(mut self) -> (Vec<Token>, Diagnostics) {
        let mut tokens = Vec::new();

        loop {
            let token = self.next_token();
            let is_eof = token.kind == TokenKind::Eof;
            tokens.push(token);
            if is_eof {
                break;
            }
        }

        (tokens, self.diagnostics)
    }

    /// Returns the next token from the source.
    fn next_token(&mut self) -> Token {
        self.skip_whitespace();

        let start = self.pos;

        if self.is_at_end() {
            return Token::new(TokenKind::Eof, self.span(start, self.pos), "");
        }

        // Placeholder: just consume one character as Unknown
        // Real implementation would match patterns here
        let c = self.advance();
        Token::new(
            TokenKind::Unknown,
            self.span(start, self.pos),
            c.to_string(),
        )
    }

    fn skip_whitespace(&mut self) {
        while !self.is_at_end() && self.peek().is_whitespace() {
            self.advance();
        }
    }

    fn is_at_end(&self) -> bool {
        self.pos >= self.source.len()
    }

    fn peek(&self) -> char {
        self.source[self.pos..].chars().next().unwrap_or('\0')
    }

    fn advance(&mut self) -> char {
        let c = self.peek();
        self.pos += c.len_utf8();
        c
    }

    fn span(&self, start: usize, end: usize) -> Span {
        Span::new(self.source_id, start as u32, end as u32)
    }
}
