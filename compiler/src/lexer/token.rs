use std::fmt;

/// Posizione nel codice sorgente
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Position {
    pub line: usize,
    pub column: usize,
}

impl Position {
    pub fn new(line: usize, column: usize) -> Self {
        Self { line, column }
    }
}

impl fmt::Display for Position {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}:{}", self.line, self.column)
    }
}

/// Tipo di token
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TokenType {
    // Parole chiave
    Let,
    Const,
    Fn,
    If,
    Else,
    For,
    While,
    Break,
    Continue,
    Return,
    Import,
    Export,
    From,
    Class,
    Interface,
    Implements,
    Try,
    Catch,
    Finally,
    Throw,
    In,
    Is,
    As,
    Type,
    Enum,
    Constructor,
    This,
    Super,
    Match,
    Case,
    Default,
    New,
    
    // Letterali
    Identifier,
    NumberLiteral,
    StringLiteral,
    True,
    False,
    Null,
    
    // Operatori
    Plus,
    Minus,
    Star,
    Slash,
    Percent,
    Equal,
    EqualEqual,
    Bang,
    BangEqual,
    Less,
    LessEqual,
    Greater,
    GreaterEqual,
    Arrow,
    Ampersand,
    AmpersandAmpersand,
    Pipe,
    PipePipe,
    
    // Punteggiatura
    LeftParen,
    RightParen,
    LeftBrace,
    RightBrace,
    LeftBracket,
    RightBracket,
    Comma,
    Dot,
    Colon,
    
    // Indentazione
    Indent,
    Dedent,
    Newline,
    
    // Fine file
    Eof,
}

impl fmt::Display for TokenType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let name = match self {
            TokenType::Let => "Let",
            TokenType::Const => "Const",
            TokenType::Fn => "Fn",
            TokenType::If => "If",
            TokenType::Else => "Else",
            TokenType::For => "For",
            TokenType::While => "While",
            TokenType::Break => "Break",
            TokenType::Continue => "Continue",
            TokenType::Return => "Return",
            TokenType::Import => "Import",
            TokenType::Export => "Export",
            TokenType::From => "From",
            TokenType::Class => "Class",
            TokenType::Interface => "Interface",
            TokenType::Implements => "Implements",
            TokenType::Try => "Try",
            TokenType::Catch => "Catch",
            TokenType::Finally => "Finally",
            TokenType::Throw => "Throw",
            TokenType::In => "In",
            TokenType::Is => "Is",
            TokenType::As => "As",
            TokenType::Type => "Type",
            TokenType::Enum => "Enum",
            TokenType::Constructor => "Constructor",
            TokenType::This => "This",
            TokenType::Super => "Super",
            TokenType::Match => "Match",
            TokenType::Case => "Case",
            TokenType::Default => "Default",
            TokenType::New => "New",
            TokenType::Identifier => "Identifier",
            TokenType::NumberLiteral => "NumberLiteral",
            TokenType::StringLiteral => "StringLiteral",
            TokenType::True => "True",
            TokenType::False => "False",
            TokenType::Null => "Null",
            TokenType::Plus => "Plus",
            TokenType::Minus => "Minus",
            TokenType::Star => "Star",
            TokenType::Slash => "Slash",
            TokenType::Percent => "Percent",
            TokenType::Equal => "Equal",
            TokenType::EqualEqual => "EqualEqual",
            TokenType::Bang => "Bang",
            TokenType::BangEqual => "BangEqual",
            TokenType::Less => "Less",
            TokenType::LessEqual => "LessEqual",
            TokenType::Greater => "Greater",
            TokenType::GreaterEqual => "GreaterEqual",
            TokenType::Arrow => "Arrow",
            TokenType::Ampersand => "Ampersand",
            TokenType::AmpersandAmpersand => "AmpersandAmpersand",
            TokenType::Pipe => "Pipe",
            TokenType::PipePipe => "PipePipe",
            TokenType::LeftParen => "LeftParen",
            TokenType::RightParen => "RightParen",
            TokenType::LeftBrace => "LeftBrace",
            TokenType::RightBrace => "RightBrace",
            TokenType::LeftBracket => "LeftBracket",
            TokenType::RightBracket => "RightBracket",
            TokenType::Comma => "Comma",
            TokenType::Dot => "Dot",
            TokenType::Colon => "Colon",
            TokenType::Indent => "Indent",
            TokenType::Dedent => "Dedent",
            TokenType::Newline => "Newline",
            TokenType::Eof => "Eof",
        };
        write!(f, "{}", name)
    }
}

/// Token del linguaggio NovaScript
#[derive(Debug, Clone, PartialEq)]
pub struct Token {
    pub token_type: TokenType,
    pub lexeme: String,
    pub position: Position,
}

impl Token {
    pub fn new(token_type: TokenType, lexeme: &str, position: Position) -> Self {
        Self {
            token_type,
            lexeme: lexeme.to_string(),
            position,
        }
    }
}

impl fmt::Display for Token {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{} '{}' at {}",
            self.token_type,
            self.lexeme,
            self.position
        )
    }
}