pub mod ast;
pub mod compiler;
pub mod error;
pub mod lexer;
pub mod parser;
pub mod semantics;
pub mod ir;
pub mod codegen;
pub mod utils;
pub mod commands;

pub use crate::compiler::Compiler;
pub use crate::error::Error;

/// Versione del compilatore NovaScript
pub const VERSION: &str = env!("CARGO_PKG_VERSION");

/// Informazioni sul compilatore NovaScript
pub fn version() -> String {
    format!("NovaScript Compiler {}", VERSION)
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_version() {
        assert!(!super::version().is_empty());
    }
}