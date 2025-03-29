use super::token::{Position, Token, TokenType};
use std::iter::Peekable;
use std::str::Chars;

/// Scanner che analizza carattere per carattere il codice sorgente
pub struct Scanner<'a> {
    source: &'a str,
    chars: Peekable<Chars<'a>>,
    start: usize,
    current: usize,
    line: usize,
    column: usize,
}

impl<'a> Scanner<'a> {
    /// Crea un nuovo scanner dal codice sorgente
    pub fn new(source: &'a str) -> Self {
        Self {
            source,
            chars: source.chars().peekable(),
            start: 0,
            current: 0,
            line: 1,
            column: 1,
        }
    }
    
    /// Restituisce la posizione corrente nel codice
    pub fn position(&self) -> Position {
        Position::new(self.line, self.column)
    }
    
    /// Verifica se lo scanner ha raggiunto la fine del codice
    pub fn is_at_end(&self) -> bool {
        self.chars.peek().is_none()
    }
    
    /// Verifica se lo scanner è a N caratteri dalla fine
    pub fn is_at_end_plus(&self, n: usize) -> bool {
        let mut count = 0;
        let mut iter = self.chars.clone();
        
        while count <= n {
            if iter.next().is_none() {
                return true;
            }
            count += 1;
        }
        
        false
    }
    
    /// Avanza di un carattere e lo restituisce
    pub fn advance(&mut self) -> char {
        let c = self.chars.next().unwrap_or('\0');
        self.current += 1;
        
        if c == '\n' {
            self.line += 1;
            self.column = 1;
        } else {
            self.column += 1;
        }
        
        c
    }
    
    /// Restituisce il carattere corrente senza avanzare
    pub fn peek(&mut self) -> char {
        *self.chars.peek().unwrap_or(&'\0')
    }
    
    /// Restituisce il carattere successivo senza avanzare
    pub fn peek_next(&mut self) -> char {
        let mut iter = self.chars.clone();
        iter.next(); // Salta il carattere corrente
        iter.next().unwrap_or('\0')
    }
    
    /// Restituisce il carattere precedente
    pub fn previous(&self) -> char {
        if self.current > 0 && self.current <= self.source.len() {
            self.source.chars().nth(self.current - 1).unwrap_or('\0')
        } else {
            '\0'
        }
    }
    
    /// Verifica se il carattere corrente corrisponde a quello atteso e avanza
    pub fn match_char(&mut self, expected: char) -> bool {
        if self.is_at_end() || self.peek() != expected {
            false
        } else {
            self.advance();
            true
        }
    }
    
    /// Restituisce il lessema corrente
    pub fn current_lexeme(&self) -> &'a str {
        &self.source[self.start..self.current]
    }
    
    /// Crea un token con il tipo specificato e il lessema corrente
    pub fn make_token(&self, token_type: TokenType) -> Token {
        Token::new(
            token_type,
            self.current_lexeme(),
            Position::new(self.line, self.column - (self.current - self.start)),
        )
    }
    
    /// Salta spazi bianchi e commenti
    pub fn skip_whitespace_and_comments(&mut self) {
        loop {
            if self.is_at_end() {
                break;
            }
            
            match self.peek() {
                // Spazi bianchi (ma non newline)
                ' ' | '\r' | '\t' => {
                    self.advance();
                }
                
                // Commento di riga
                '/' if !self.is_at_end_plus(1) && self.peek_next() == '/' => {
                    // Consuma tutto fino alla fine della riga
                    while !self.is_at_end() && self.peek() != '\n' {
                        self.advance();
                    }
                }
                
                // Commento multi-linea
                '/' if !self.is_at_end_plus(1) && self.peek_next() == '*' => {
                    // Consuma /* iniziale
                    self.advance();
                    self.advance();
                    
                    // Consuma il contenuto del commento finché non troviamo */
                    let mut nesting = 1;
                    while !self.is_at_end() && nesting > 0 {
                        if self.peek() == '/' && !self.is_at_end_plus(1) && self.peek_next() == '*' {
                            self.advance();
                            self.advance();
                            nesting += 1;
                        } else if self.peek() == '*' && !self.is_at_end_plus(1) && self.peek_next() == '/' {
                            self.advance();
                            self.advance();
                            nesting -= 1;
                        } else {
                            self.advance();
                        }
                    }
                }
                
                // Altri caratteri
                _ => break,
            }
        }
        
        self.start = self.current;
    }
    
    /// Calcola l'indentazione corrente (numero di spazi all'inizio della riga)
    pub fn calculate_indent(&mut self) -> usize {
        let mut indent = 0;
        
        while !self.is_at_end() {
            match self.peek() {
                ' ' => {
                    indent += 1;
                    self.advance();
                }
                '\t' => {
                    // Consideriamo un tab come 4 spazi
                    indent += 4;
                    self.advance();
                }
                _ => break,
            }
        }
        
        self.start = self.current;
        indent
    }
}