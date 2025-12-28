//! Shared IR utilities for the Ted compiler.
//!
//! This crate provides common types used across the compiler pipeline:
//! - Source location tracking (`Span`, `SourceId`)
//! - Node identifiers (`NodeId`)

/// Identifies a source file in the compilation.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct SourceId(pub u32);

/// Identifies a node in the AST or IR.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct NodeId(pub u32);

/// A span representing a range in source code.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Span {
    /// The source file this span belongs to.
    pub source: SourceId,
    /// Start byte offset (inclusive).
    pub start: u32,
    /// End byte offset (exclusive).
    pub end: u32,
}

impl Span {
    /// Creates a new span.
    #[must_use]
    pub fn new(source: SourceId, start: u32, end: u32) -> Self {
        Self { source, start, end }
    }

    /// Creates a dummy span for generated code or testing.
    #[must_use]
    pub fn dummy() -> Self {
        Self {
            source: SourceId(0),
            start: 0,
            end: 0,
        }
    }

    /// Returns the length of this span in bytes.
    #[must_use]
    pub fn len(&self) -> u32 {
        self.end - self.start
    }

    /// Returns true if this span is empty.
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.start == self.end
    }
}
