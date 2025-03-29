use crate::lexer::Position;
use std::fmt;
use thiserror::Error;

/// Tipi di errore del compilatore NovaScript
#[derive(Debug, Error, PartialEq, Eq)]
pub enum ErrorKind {
    #[error("Errore di sintassi")]
    SyntaxError,
    
    #[error("Errore di tipo")]
    TypeError,
    
    #[error("Simbolo non definito")]
    UndefinedSymbol,
    
    #[error("Errore di I/O")]
    IoError,
    
    #[error("Errore interno")]
    InternalError,
    
    #[error("Errore di runtime")]
    RuntimeError,
}

/// Errore del compilatore NovaScript
#[derive(Debug)]
pub struct Error {
    pub kind: ErrorKind,
    pub message: String,
    pub position: Option<Position>,
}

impl Error {
    /// Crea un nuovo errore
    pub fn new(kind: ErrorKind, message: String, position: Position) -> Self {
        Self {
            kind,
            message,
            position: Some(position),
        }
    }
    
    /// Crea un errore senza posizione nel codice
    pub fn without_position(kind: ErrorKind, message: String) -> Self {
        Self {
            kind,
            message,
            position: None,
        }
    }
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match &self.position {
            Some(pos) => write!(f, "{} a {}:{}: {}", self.kind, pos.line, pos.column, self.message),
            None => write!(f, "{}: {}", self.kind, self.message),
        }
    }
}

impl std::error::Error for Error {}

/// Conversione di errori I/O in errori NovaScript
impl From<std::io::Error> for Error {
    fn from(error: std::io::Error) -> Self {
        Error::without_position(
            ErrorKind::IoError,
            format!("Errore I/O: {}", error),
        )
    }
}

/// Tipo Result specifico per NovaScript
pub type Result<T> = std::result::Result<T, Error>;