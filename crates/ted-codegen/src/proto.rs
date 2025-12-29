//! Prototype source parser for codegen.

use crate::Program;
use std::fmt;

#[derive(Debug, Clone)]
pub struct ProtoError {
    message: String,
    line: usize,
    column: usize,
}

impl ProtoError {
    fn new(message: impl Into<String>, line: usize, column: usize) -> Self {
        Self {
            message: message.into(),
            line,
            column,
        }
    }
}

impl fmt::Display for ProtoError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "proto parse error at {}:{}: {}",
            self.line, self.column, self.message
        )
    }
}

impl std::error::Error for ProtoError {}

#[derive(Debug, Clone, PartialEq, Eq)]
enum TokenKind {
    Ident(String),
    Int(String),
    Str(String),
    Punct(char),
}

#[derive(Debug, Clone)]
struct Token {
    kind: TokenKind,
    line: usize,
    column: usize,
}

pub fn parse_program(source: &str) -> Result<Program, ProtoError> {
    let mut cursor = Cursor::new(source);
    let mut module_name = None;
    let mut prints = Vec::new();

    while let Some(token) = cursor.next_token()? {
        match &token.kind {
            TokenKind::Ident(name) if name == "mod" => {
                let next = cursor.next_token()?.ok_or_else(|| {
                    ProtoError::new("expected module name", token.line, token.column)
                })?;
                match next.kind {
                    TokenKind::Ident(name) => {
                        if module_name.is_none() {
                            module_name = Some(name);
                        }
                    }
                    _ => {
                        return Err(ProtoError::new(
                            "expected module name after `mod`",
                            next.line,
                            next.column,
                        ));
                    }
                }
            }
            TokenKind::Ident(name) if name == "print" => {
                let value = parse_print_call(&mut cursor, token.line, token.column)?;
                prints.push(value);
            }
            _ => {}
        }
    }

    let module_name = module_name.ok_or_else(|| ProtoError::new("missing module name", 1, 1))?;

    Ok(Program {
        module_name,
        prints,
    })
}

fn parse_print_call(
    cursor: &mut Cursor<'_>,
    line: usize,
    column: usize,
) -> Result<String, ProtoError> {
    expect_punct(cursor, '(', "expected '(' after `print`", line, column)?;

    let value_token = cursor
        .next_token()?
        .ok_or_else(|| ProtoError::new("expected print value", line, column))?;

    let value = match value_token.kind {
        TokenKind::Str(text) => text,
        TokenKind::Int(text) => text,
        _ => {
            return Err(ProtoError::new(
                "print only supports literal strings or integers",
                value_token.line,
                value_token.column,
            ));
        }
    };

    expect_punct(
        cursor,
        ')',
        "expected ')' after print value",
        value_token.line,
        value_token.column,
    )?;

    if let Some(next) = cursor.peek_token()? {
        if matches!(next.kind, TokenKind::Punct(';')) {
            cursor.next_token()?;
        }
    }

    Ok(value)
}

fn expect_punct(
    cursor: &mut Cursor<'_>,
    ch: char,
    message: &str,
    line: usize,
    column: usize,
) -> Result<(), ProtoError> {
    let token = cursor
        .next_token()?
        .ok_or_else(|| ProtoError::new(message, line, column))?;
    match token.kind {
        TokenKind::Punct(found) if found == ch => Ok(()),
        _ => Err(ProtoError::new(message, token.line, token.column)),
    }
}

struct Cursor<'a> {
    src: &'a [u8],
    index: usize,
    line: usize,
    column: usize,
    peeked: Option<Token>,
}

impl<'a> Cursor<'a> {
    fn new(source: &'a str) -> Self {
        Self {
            src: source.as_bytes(),
            index: 0,
            line: 1,
            column: 1,
            peeked: None,
        }
    }

    fn peek_token(&mut self) -> Result<Option<Token>, ProtoError> {
        if self.peeked.is_none() {
            self.peeked = self.next_token_inner()?;
        }
        Ok(self.peeked.clone())
    }

    fn next_token(&mut self) -> Result<Option<Token>, ProtoError> {
        if self.peeked.is_some() {
            return Ok(self.peeked.take());
        }
        self.next_token_inner()
    }

    fn next_token_inner(&mut self) -> Result<Option<Token>, ProtoError> {
        self.skip_ws_and_comments()?;
        let start_line = self.line;
        let start_column = self.column;

        let ch = match self.bump_byte() {
            Some(ch) => ch,
            None => return Ok(None),
        };

        let kind = match ch {
            b'a'..=b'z' | b'A'..=b'Z' | b'_' => {
                let ident = self.consume_while(|c| is_ident_char(c), ch);
                TokenKind::Ident(ident)
            }
            b'0'..=b'9' => {
                let value = self.consume_while(|c| is_number_char(c), ch);
                TokenKind::Int(value)
            }
            b'"' => {
                let text = self.consume_string(start_line, start_column)?;
                TokenKind::Str(text)
            }
            b'(' | b')' | b';' | b'{' | b'}' | b',' => TokenKind::Punct(ch as char),
            _ => {
                return Err(ProtoError::new(
                    format!("unexpected character '{}'", ch as char),
                    start_line,
                    start_column,
                ));
            }
        };

        Ok(Some(Token {
            kind,
            line: start_line,
            column: start_column,
        }))
    }

    fn skip_ws_and_comments(&mut self) -> Result<(), ProtoError> {
        loop {
            self.consume_while(|c| is_whitespace(c), 0);

            if self.starts_with(b"//") {
                self.consume_line_comment();
                continue;
            }

            if self.starts_with(b"/*") {
                self.consume_block_comment()?;
                continue;
            }

            break;
        }

        Ok(())
    }

    fn consume_line_comment(&mut self) {
        self.bump_byte();
        self.bump_byte();
        while let Some(ch) = self.peek_byte() {
            if ch == b'\n' {
                break;
            }
            self.bump_byte();
        }
    }

    fn consume_block_comment(&mut self) -> Result<(), ProtoError> {
        let start_line = self.line;
        let start_column = self.column;
        self.bump_byte();
        self.bump_byte();
        while let Some(_) = self.peek_byte() {
            if self.starts_with(b"*/") {
                self.bump_byte();
                self.bump_byte();
                return Ok(());
            }
            self.bump_byte();
        }

        Err(ProtoError::new(
            "unterminated block comment",
            start_line,
            start_column,
        ))
    }

    fn consume_string(&mut self, line: usize, column: usize) -> Result<String, ProtoError> {
        let mut out = String::new();
        while let Some(ch) = self.bump_byte() {
            match ch {
                b'"' => return Ok(out),
                b'\\' => {
                    let escaped = match self.bump_byte() {
                        Some(b'"') => '"',
                        Some(b'\\') => '\\',
                        Some(b'n') => '\n',
                        Some(b'r') => '\r',
                        Some(b't') => '\t',
                        Some(other) => {
                            return Err(ProtoError::new(
                                format!("unsupported escape \\{}", other as char),
                                self.line,
                                self.column,
                            ));
                        }
                        None => {
                            return Err(ProtoError::new(
                                "unterminated string literal",
                                line,
                                column,
                            ));
                        }
                    };
                    out.push(escaped);
                }
                other => out.push(other as char),
            }
        }

        Err(ProtoError::new("unterminated string literal", line, column))
    }

    fn consume_while(&mut self, predicate: fn(u8) -> bool, first: u8) -> String {
        let mut out = String::new();
        if first != 0 {
            out.push(first as char);
        }
        while let Some(ch) = self.peek_byte() {
            if !predicate(ch) {
                break;
            }
            out.push(ch as char);
            self.bump_byte();
        }
        out
    }

    fn bump_byte(&mut self) -> Option<u8> {
        let byte = *self.src.get(self.index)?;
        self.index += 1;
        if byte == b'\n' {
            self.line += 1;
            self.column = 1;
        } else {
            self.column += 1;
        }
        Some(byte)
    }

    fn peek_byte(&self) -> Option<u8> {
        self.src.get(self.index).copied()
    }

    fn starts_with(&self, pat: &[u8]) -> bool {
        self.src[self.index..].starts_with(pat)
    }
}

fn is_whitespace(ch: u8) -> bool {
    matches!(ch, b' ' | b'\t' | b'\n' | b'\r')
}

fn is_ident_char(ch: u8) -> bool {
    ch.is_ascii_alphanumeric() || ch == b'_'
}

fn is_number_char(ch: u8) -> bool {
    ch.is_ascii_alphanumeric() || ch == b'_'
}

#[cfg(test)]
mod tests {
    use super::parse_program;

    #[test]
    fn parses_print_string() {
        let source = r#"
            mod hello {
                print("Hello, Ted!");
            }
        "#;
        let program = parse_program(source).expect("program should parse");
        assert_eq!(program.module_name, "hello");
        assert_eq!(program.prints, vec!["Hello, Ted!".to_string()]);
    }

    #[test]
    fn parses_print_int() {
        let source = r#"
            mod hello {
                print(42);
            }
        "#;
        let program = parse_program(source).expect("program should parse");
        assert_eq!(program.prints, vec!["42".to_string()]);
    }
}
