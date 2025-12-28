//! Diagnostic reporting for the Ted compiler.
//!
//! Provides structured error messages with source locations.

use ted_ir::Span;

/// The severity level of a diagnostic.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Severity {
    /// A fatal error that prevents compilation.
    Error,
    /// A warning that doesn't prevent compilation.
    Warning,
    /// An informational note.
    Note,
}

/// A compiler diagnostic with location and message.
#[derive(Debug, Clone)]
pub struct Diagnostic {
    /// The severity of this diagnostic.
    pub severity: Severity,
    /// The primary message.
    pub message: String,
    /// The source location where the issue occurred.
    pub span: Span,
    /// Optional labels for additional context.
    pub labels: Vec<Label>,
}

/// A labeled span providing additional context.
#[derive(Debug, Clone)]
pub struct Label {
    /// The span this label points to.
    pub span: Span,
    /// The label message.
    pub message: String,
}

impl Diagnostic {
    /// Creates a new error diagnostic.
    #[must_use]
    pub fn error(message: impl Into<String>, span: Span) -> Self {
        Self {
            severity: Severity::Error,
            message: message.into(),
            span,
            labels: Vec::new(),
        }
    }

    /// Creates a new warning diagnostic.
    #[must_use]
    pub fn warning(message: impl Into<String>, span: Span) -> Self {
        Self {
            severity: Severity::Warning,
            message: message.into(),
            span,
            labels: Vec::new(),
        }
    }

    /// Adds a label to this diagnostic.
    #[must_use]
    pub fn with_label(mut self, span: Span, message: impl Into<String>) -> Self {
        self.labels.push(Label {
            span,
            message: message.into(),
        });
        self
    }
}

/// A collection of diagnostics from a compilation phase.
#[derive(Debug, Default)]
pub struct Diagnostics {
    items: Vec<Diagnostic>,
}

impl Diagnostics {
    /// Creates an empty diagnostics collection.
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }

    /// Adds a diagnostic.
    pub fn add(&mut self, diagnostic: Diagnostic) {
        self.items.push(diagnostic);
    }

    /// Returns true if there are any errors.
    #[must_use]
    pub fn has_errors(&self) -> bool {
        self.items.iter().any(|d| d.severity == Severity::Error)
    }

    /// Returns an iterator over all diagnostics.
    pub fn iter(&self) -> impl Iterator<Item = &Diagnostic> {
        self.items.iter()
    }
}
