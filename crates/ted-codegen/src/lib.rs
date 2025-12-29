//! Code generation backends for the Ted compiler.

use std::fmt;

mod proto;

pub use proto::ProtoError;

/// Compiler-ready program input for code generation.
#[derive(Debug, Clone)]
pub struct Program {
    pub module_name: String,
    pub prints: Vec<String>,
}

impl Program {
    /// Builds a codegen program from source text.
    ///
    /// TODO: Replace this with MIR/HIR lowering once available.
    pub fn from_source(source: &str) -> Result<Self, ProtoError> {
        proto::parse_program(source)
    }

    /// Builds a codegen program from a parsed module.
    ///
    /// TODO: Replace this with MIR/HIR lowering once available.
    pub fn from_parsed(module: &ted_parser::Module) -> Self {
        Self {
            module_name: module.name.clone(),
            prints: Vec::new(),
        }
    }
}

/// Optimization levels for code generation.
#[derive(Debug, Clone, Copy)]
pub enum OptLevel {
    None,
    Speed,
    SpeedAndSize,
}

impl Default for OptLevel {
    fn default() -> Self {
        Self::None
    }
}

/// Codegen configuration shared across backends.
#[derive(Debug, Clone)]
pub struct CodegenConfig {
    pub opt_level: OptLevel,
    pub target: Option<String>,
}

impl Default for CodegenConfig {
    fn default() -> Self {
        Self {
            opt_level: OptLevel::None,
            target: None,
        }
    }
}

/// A compiled object file.
#[derive(Debug)]
pub struct ObjectFile {
    pub bytes: Vec<u8>,
}

/// Codegen errors.
#[derive(Debug)]
pub enum CodegenError {
    Unsupported(String),
    Backend(String),
}

impl fmt::Display for CodegenError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            CodegenError::Unsupported(msg) => write!(f, "unsupported: {msg}"),
            CodegenError::Backend(msg) => write!(f, "backend error: {msg}"),
        }
    }
}

impl std::error::Error for CodegenError {}

#[cfg(feature = "cranelift")]
pub mod cranelift;

#[cfg(all(test, feature = "cranelift"))]
mod tests {
    use super::*;

    #[test]
    fn emits_object_file() {
        let program = Program {
            module_name: "test".to_string(),
            prints: vec!["hello".to_string()],
        };
        let backend = cranelift::CraneliftBackend::new(CodegenConfig::default());
        let object = backend
            .compile_object(&program)
            .expect("cranelift backend should emit object bytes");
        assert!(!object.bytes.is_empty());
    }
}
