use crate::error::LoxError;
use crate::token::*;

struct Scanner {
    source: Vec<char>,
    tokens: Vec<Token>,
    // the first character in the lexeme being scanned
    start: usize,
    // the character currently being considered
    current: usize,
    line: usize,
}

impl Scanner {
    fn new(source: String) -> Self {
        Self {
            source: source.chars().collect(),
            tokens: Vec::new(),
            start: 0,
            current: 0,
            line: 1,
        }
    }

    fn scan_tokens(&mut self) -> Result<&Vec<Token>, LoxError> {
        let mut had_error: Option<LoxError> = None;
        while !self.is_end() {
            self.start = self.current;
            match self.scan_token() {
                Ok(_) => {}
                Err(e) => {
                    e.report("".to_string());
                    had_error = Some(e);
                }
            }
        }

        // add "end of file" token
        self.tokens.push(Token::new(
            TokenType::Eof,
            String::new(),
            Some(Literal::Nil),
            self.line,
        ));
        if had_error.is_some() {
            Err(had_error.unwrap())
        } else {
            Ok(&self.tokens)
        }
    }

    fn is_end(&self) -> bool {
        self.current >= self.source.len()
    }

    fn scan_token(&mut self) -> Result<(), LoxError> {
        let c: char = self.source[self.current as usize];
        self.current += 1;
        match c {
            '(' => self.add_token(TokenType::LeftParen),
            ')' => self.add_token(TokenType::RightParen),
            '{' => self.add_token(TokenType::LeftBrace),
            '}' => self.add_token(TokenType::RightBrace),
            ',' => self.add_token(TokenType::Comma),
            '.' => self.add_token(TokenType::Dot),
            '+' => self.add_token(TokenType::Pluse),
            '-' => self.add_token(TokenType::Minus),
            '*' => self.add_token(TokenType::Star),
            ';' => self.add_token(TokenType::Semicolon),
            '!' => {
                let tkt = if self.next_is('=') {
                    TokenType::BangEqual
                } else {
                    TokenType::Bang
                };
                self.add_token(tkt);
            }
            '=' => {
                let tkt = if self.next_is('=') {
                    TokenType::EqualEqual
                } else {
                    TokenType::Equal
                };
                self.add_token(tkt);
            }
            '<' => {
                let tkt = if self.next_is('=') {
                    TokenType::LessEqual
                } else {
                    TokenType::Less
                };
                self.add_token(tkt);
            }
            '>' => {
                let tkt = if self.next_is('=') {
                    TokenType::GreaterEqual
                } else {
                    TokenType::Greater
                };
                self.add_token(tkt);
            }
            _ => {
                return Err(LoxError::error(
                    self.line,
                    "Unexpected character".to_string(),
                ))
            }
        }
        Ok(())
    }

    fn add_token(&mut self, kind: TokenType) {
        let text: String = self.source[self.start..self.current].iter().collect();
        self.tokens.push(Token::new(kind, text, None, self.line));
    }

    fn next_is(&mut self, expected: char) -> bool {
        if self.is_end() {
            return false;
        }
        if self.source[self.current] != expected {
            return false;
        }
        self.current += 1;
        true
    }
}
