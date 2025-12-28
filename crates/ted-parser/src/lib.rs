//! Parser and AST for the Ted language.
//!
//! Parses tokens into an abstract syntax tree.

use ted_diagnostics::Diagnostics;
use ted_ir::{NodeId, Span};
use ted_lexer::Token;

/// A parsed Ted module.
#[derive(Debug)]
pub struct Module {
    /// The module's unique ID.
    pub id: NodeId,
    /// The module name.
    pub name: String,
    /// The span of the module declaration.
    pub span: Span,
    /// Items declared in this module.
    pub items: Vec<Item>,
}

/// An item in a module.
#[derive(Debug)]
pub enum Item {
    /// A signal declaration.
    Signal(Signal),
}

/// A signal declaration.
#[derive(Debug)]
pub struct Signal {
    /// The signal's unique ID.
    pub id: NodeId,
    /// The signal name.
    pub name: String,
    /// The span of the declaration.
    pub span: Span,
}

/// The parser for Ted source code.
#[allow(dead_code)]
pub struct Parser {
    tokens: Vec<Token>,
    pos: usize,
    next_node_id: u32,
    diagnostics: Diagnostics,
}

impl Parser {
    /// Creates a new parser from tokens.
    #[must_use]
    pub fn new(tokens: Vec<Token>) -> Self {
        Self {
            tokens,
            pos: 0,
            next_node_id: 0,
            diagnostics: Diagnostics::new(),
        }
    }

    /// Parses the tokens into a module.
    pub fn parse(mut self) -> (Option<Module>, Diagnostics) {
        // Placeholder: return an empty module
        let module = Module {
            id: self.next_id(),
            name: "main".to_string(),
            span: Span::dummy(),
            items: Vec::new(),
        };

        (Some(module), self.diagnostics)
    }

    fn next_id(&mut self) -> NodeId {
        let id = NodeId(self.next_node_id);
        self.next_node_id += 1;
        id
    }
}

/// Convenience function to parse source code directly.
pub fn parse_source(source_id: ted_ir::SourceId, source: &str) -> (Option<Module>, Diagnostics) {
    let (tokens, mut diags) = ted_lexer::Lexer::new(source_id, source).tokenize();

    if diags.has_errors() {
        return (None, diags);
    }

    let (module, parse_diags) = Parser::new(tokens).parse();

    for d in parse_diags.iter() {
        diags.add(d.clone());
    }

    (module, diags)
}
