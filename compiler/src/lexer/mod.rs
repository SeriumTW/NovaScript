mod token;
mod scanner;

pub use token::{Token, TokenType, Position};
pub use scanner::Scanner;

use crate::error::{Error, ErrorKind};
use std::iter::Peekable;
use std::str::Chars;

/// Lexer per NovaScript che converte una stringa di codice sorgente in un flusso di token
pub struct Lexer<'a> {
    scanner: Scanner<'a>,
    indent_stack: Vec<usize>,
    tokens_buffer: Vec<Token>,
    at_line_start: bool,
    current_indent: usize,
}

impl<'a> Lexer<'a> {
    /// Crea un nuovo lexer dal codice sorgente
    pub fn new(source: &'a str) -> Self {
        Self {
            scanner: Scanner::new(source),
            indent_stack: vec![0], // Lo stack inizia con l'indentazione 0
            tokens_buffer: Vec::new(),
            at_line_start: true,
            current_indent: 0,
        }
    }
    
    /// Scansiona e restituisce tutti i token dal codice sorgente
    pub fn tokenize(&mut self) -> Result<Vec<Token>, Error> {
        let mut tokens = Vec::new();
        
        loop {
            match self.next_token() {
                Ok(token) => {
                    let is_eof = token.token_type == TokenType::Eof;
                    tokens.push(token);
                    if is_eof {
                        break;
                    }
                },
                Err(e) => return Err(e),
            }
        }
        
        Ok(tokens)
    }
    
    /// Restituisce il prossimo token
    pub fn next_token(&mut self) -> Result<Token, Error> {
        // Se ci sono token nel buffer, restituiscili prima
        if !self.tokens_buffer.is_empty() {
            return Ok(self.tokens_buffer.remove(0));
        }
        
        // Salta spazi bianchi e commenti
        self.scanner.skip_whitespace_and_comments();
        
        // Se siamo all'inizio di una linea, gestisci l'indentazione
        if self.at_line_start {
            self.at_line_start = false;
            return self.handle_indentation();
        }
        
        // Rileva il fine riga
        if self.scanner.is_at_end() {
            // Genera token DEDENT per indentazioni rimanenti
            if !self.indent_stack.is_empty() && self.indent_stack.last().unwrap() > &0 {
                while self.indent_stack.len() > 1 {
                    self.indent_stack.pop();
                    self.tokens_buffer.push(Token::new(
                        TokenType::Dedent,
                        "",
                        self.scanner.position(),
                    ));
                }
            }
            return Ok(Token::new(
                TokenType::Eof,
                "",
                self.scanner.position(),
            ));
        }
        
        // Se troviamo un carattere newline
        if self.scanner.match_char('\n') {
            self.at_line_start = true;
            return Ok(Token::new(
                TokenType::Newline,
                "\n",
                self.scanner.position(),
            ));
        }
        
        // Scansiona il prossimo token
        self.scan_token()
    }
    
    /// Gestisce l'indentazione all'inizio di una linea
    fn handle_indentation(&mut self) -> Result<Token, Error> {
        // Calcola l'indentazione corrente
        let current_indent = self.scanner.calculate_indent();
        let previous_indent = *self.indent_stack.last().unwrap();
        
        // Aggiorna l'indentazione corrente
        self.current_indent = current_indent;
        
        // Genera token INDENT o DEDENT in base alla differenza di indentazione
        if current_indent > previous_indent {
            // Indentazione aumentata
            self.indent_stack.push(current_indent);
            return Ok(Token::new(
                TokenType::Indent,
                &" ".repeat(current_indent),
                self.scanner.position(),
            ));
        } else if current_indent < previous_indent {
            // Indentazione diminuita
            while !self.indent_stack.is_empty() && self.indent_stack.last().unwrap() > &current_indent {
                self.indent_stack.pop();
                self.tokens_buffer.push(Token::new(
                    TokenType::Dedent,
                    "",
                    self.scanner.position(),
                ));
            }
            
            if self.indent_stack.is_empty() || self.indent_stack.last().unwrap() != &current_indent {
                return Err(Error::new(
                    ErrorKind::SyntaxError,
                    "Indentazione non valida".to_string(),
                    self.scanner.position(),
                ));
            }
            
            return Ok(self.tokens_buffer.remove(0));
        }
        
        // Indentazione non cambiata, continua con il prossimo token
        self.scan_token()
    }
    
    /// Scansiona e restituisce il prossimo token
    fn scan_token(&mut self) -> Result<Token, Error> {
        // Identifica il carattere attuale
        let c = self.scanner.advance();
        
        // Corrispondenza in base al carattere
        match c {
            // Operatori a carattere singolo
            '+' => Ok(self.scanner.make_token(TokenType::Plus)),
            '-' => Ok(self.scanner.make_token(TokenType::Minus)),
            '*' => Ok(self.scanner.make_token(TokenType::Star)),
            '/' => Ok(self.scanner.make_token(TokenType::Slash)),
            '%' => Ok(self.scanner.make_token(TokenType::Percent)),
            '(' => Ok(self.scanner.make_token(TokenType::LeftParen)),
            ')' => Ok(self.scanner.make_token(TokenType::RightParen)),
            '{' => Ok(self.scanner.make_token(TokenType::LeftBrace)),
            '}' => Ok(self.scanner.make_token(TokenType::RightBrace)),
            '[' => Ok(self.scanner.make_token(TokenType::LeftBracket)),
            ']' => Ok(self.scanner.make_token(TokenType::RightBracket)),
            ',' => Ok(self.scanner.make_token(TokenType::Comma)),
            '.' => Ok(self.scanner.make_token(TokenType::Dot)),
            ':' => Ok(self.scanner.make_token(TokenType::Colon)),
            
            // Operatori composti
            '=' => {
                if self.scanner.match_char('=') {
                    Ok(self.scanner.make_token(TokenType::EqualEqual))
                } else {
                    Ok(self.scanner.make_token(TokenType::Equal))
                }
            },
            '!' => {
                if self.scanner.match_char('=') {
                    Ok(self.scanner.make_token(TokenType::BangEqual))
                } else {
                    Ok(self.scanner.make_token(TokenType::Bang))
                }
            },
            '<' => {
                if self.scanner.match_char('=') {
                    Ok(self.scanner.make_token(TokenType::LessEqual))
                } else {
                    Ok(self.scanner.make_token(TokenType::Less))
                }
            },
            '>' => {
                if self.scanner.match_char('=') {
                    Ok(self.scanner.make_token(TokenType::GreaterEqual))
                } else {
                    Ok(self.scanner.make_token(TokenType::Greater))
                }
            },
            
            // Letterali stringa
            '"' | '\'' => self.string(),
            
            // Identificatori e parole chiave
            c if c.is_alphabetic() || c == '_' => self.identifier_or_keyword(),
            
            // Numeri
            c if c.is_digit(10) => self.number(),
            
            // Caratteri non riconosciuti
            _ => Err(Error::new(
                ErrorKind::SyntaxError,
                format!("Carattere non riconosciuto: '{}'", c),
                self.scanner.position(),
            )),
        }
    }
    
    /// Scansiona un letterale stringa
    fn string(&mut self) -> Result<Token, Error> {
        let quote = self.scanner.previous();
        let start_pos = self.scanner.position();
        
        while !self.scanner.is_at_end() && self.scanner.peek() != quote {
            // Gestisci sequenze di escape
            if self.scanner.peek() == '\\' {
                self.scanner.advance(); // Consuma il backslash
                if self.scanner.is_at_end() {
                    break;
                }
                self.scanner.advance(); // Consuma il carattere di escape
            } else {
                self.scanner.advance();
            }
        }
        
        if self.scanner.is_at_end() {
            return Err(Error::new(
                ErrorKind::SyntaxError,
                "Stringa non terminata".to_string(),
                start_pos,
            ));
        }
        
        // Consuma la chiusura della stringa
        self.scanner.advance();
        
        // Estrai il contenuto della stringa senza le virgolette
        let lexeme = self.scanner.current_lexeme();
        let content = &lexeme[1..lexeme.len() - 1];
        
        Ok(Token::new(
            TokenType::StringLiteral,
            content,
            start_pos,
        ))
    }
    
    /// Scansiona un identificatore o una parola chiave
    fn identifier_or_keyword(&mut self) -> Result<Token, Error> {
        while !self.scanner.is_at_end() && 
              (self.scanner.peek().is_alphanumeric() || self.scanner.peek() == '_') {
            self.scanner.advance();
        }
        
        let lexeme = self.scanner.current_lexeme();
        let token_type = match lexeme {
            // Parole chiave
            "let" => TokenType::Let,
            "const" => TokenType::Const,
            "fn" => TokenType::Fn,
            "if" => TokenType::If,
            "else" => TokenType::Else,
            "for" => TokenType::For,
            "while" => TokenType::While,
            "return" => TokenType::Return,
            "true" => TokenType::True,
            "false" => TokenType::False,
            "null" => TokenType::Null,
            "import" => TokenType::Import,
            "export" => TokenType::Export,
            "from" => TokenType::From,
            "class" => TokenType::Class,
            "interface" => TokenType::Interface,
            "implements" => TokenType::Implements,
            "try" => TokenType::Try,
            "catch" => TokenType::Catch,
            "finally" => TokenType::Finally,
            "throw" => TokenType::Throw,
            "break" => TokenType::Break,
            "continue" => TokenType::Continue,
            "in" => TokenType::In,
            "is" => TokenType::Is,
            "as" => TokenType::As,
            "type" => TokenType::Type,
            "enum" => TokenType::Enum,
            "constructor" => TokenType::Constructor,
            "this" => TokenType::This,
            "super" => TokenType::Super,
            "match" => TokenType::Match,
            "case" => TokenType::Case,
            "default" => TokenType::Default,
            "new" => TokenType::New,
            // Altrimenti è un identificatore
            _ => TokenType::Identifier,
        };
        
        Ok(Token::new(
            token_type,
            lexeme,
            self.scanner.position(),
        ))
    }
    
    /// Scansiona un letterale numerico
    fn number(&mut self) -> Result<Token, Error> {
        // Consuma la parte intera
        while !self.scanner.is_at_end() && self.scanner.peek().is_digit(10) {
            self.scanner.advance();
        }
        
        // Controlla se c'è una parte decimale
        if !self.scanner.is_at_end() && self.scanner.peek() == '.' && 
           !self.scanner.is_at_end_plus(1) && self.scanner.peek_next().is_digit(10) {
            // Consuma il punto
            self.scanner.advance();
            
            // Consuma la parte decimale
            while !self.scanner.is_at_end() && self.scanner.peek().is_digit(10) {
                self.scanner.advance();
            }
        }
        
        Ok(Token::new(
            TokenType::NumberLiteral,
            self.scanner.current_lexeme(),
            self.scanner.position(),
        ))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_basic_tokens() {
        let source = "let x = 42";
        let mut lexer = Lexer::new(source);
        let tokens = lexer.tokenize().unwrap();
        
        assert_eq!(tokens.len(), 5); // let, x, =, 42, EOF
        assert_eq!(tokens[0].token_type, TokenType::Let);
        assert_eq!(tokens[1].token_type, TokenType::Identifier);
        assert_eq!(tokens[1].lexeme, "x");
        assert_eq!(tokens[2].token_type, TokenType::Equal);
        assert_eq!(tokens[3].token_type, TokenType::NumberLiteral);
        assert_eq!(tokens[3].lexeme, "42");
        assert_eq!(tokens[4].token_type, TokenType::Eof);
    }
    
    #[test]
    fn test_indentation() {
        let source = "if x\n  y = 10\n  z = 20\nw = 30";
        let mut lexer = Lexer::new(source);
        let tokens = lexer.tokenize().unwrap();
        
        // Verifica la presenza dei token INDENT e DEDENT
        let token_types: Vec<TokenType> = tokens.iter().map(|t| t.token_type).collect();
        assert!(token_types.contains(&TokenType::Indent));
        assert!(token_types.contains(&TokenType::Dedent));
    }
    
    #[test]
    fn test_string_literal() {
        let source = "\"hello world\"";
        let mut lexer = Lexer::new(source);
        let tokens = lexer.tokenize().unwrap();
        
        assert_eq!(tokens.len(), 2); // string, EOF
        assert_eq!(tokens[0].token_type, TokenType::StringLiteral);
        assert_eq!(tokens[0].lexeme, "hello world");
    }
}