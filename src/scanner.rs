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
            '(' => self.add_token(TokenType::LeftParen, Literal::Empty),
            ')' => self.add_token(TokenType::RightParen, Literal::Empty),
            '{' => self.add_token(TokenType::LeftBrace, Literal::Empty),
            '}' => self.add_token(TokenType::RightBrace, Literal::Empty),
            ',' => self.add_token(TokenType::Comma, Literal::Empty),
            '.' => self.add_token(TokenType::Dot, Literal::Empty),
            '+' => self.add_token(TokenType::Pluse, Literal::Empty),
            '-' => self.add_token(TokenType::Minus, Literal::Empty),
            '*' => self.add_token(TokenType::Star, Literal::Empty),
            ';' => self.add_token(TokenType::Semicolon, Literal::Empty),
            '!' => {
                let tkt = if self.match_next('=') {
                    TokenType::BangEqual
                } else {
                    TokenType::Bang
                };
                self.add_token(tkt, Literal::Empty);
            }
            '=' => {
                let tkt = if self.match_next('=') {
                    TokenType::EqualEqual
                } else {
                    TokenType::Equal
                };
                self.add_token(tkt, Literal::Empty);
            }
            '<' => {
                let tkt = if self.match_next('=') {
                    TokenType::LessEqual
                } else {
                    TokenType::Less
                };
                self.add_token(tkt, Literal::Empty);
            }
            '>' => {
                let tkt = if self.match_next('=') {
                    TokenType::GreaterEqual
                } else {
                    TokenType::Greater
                };
                self.add_token(tkt, Literal::Empty);
            }
            '/' => {
                if self.match_next('/') {
                    // 注释行
                    while self.peek() != '\n' && !self.is_end() {
                        self.current += 1;
                    }
                } else {
                    self.add_token(TokenType::Slash, Literal::Empty);
                }
            }
            '\n' => self.line += 1, // '\n'也是 whitespace
            c if c.is_ascii_whitespace() => {}
            '"' => self.match_string()?,
            c if c.is_numeric() => self.match_number()?,
            _ => return Err(LoxError::new(self.line, "Unexpected character".to_string())),
        }
        Ok(())
    }

    fn add_token(&mut self, kind: TokenType, literal: Literal) {
        let text: String = self.source[self.start..self.current].iter().collect();

        let literal = match literal {
            Literal::Empty => None,
            l => Some(l),
        };
        self.tokens.push(Token::new(kind, text, literal, self.line));
    }

    fn match_next(&mut self, expected: char) -> bool {
        if self.is_end() {
            return false;
        }
        if self.source[self.current] != expected {
            return false;
        }
        self.current += 1;
        true
    }

    fn peek(&self) -> char {
        if self.is_end() {
            '\0'
        } else {
            self.source[self.current]
        }
    }
    fn peek_next(&self) -> char {
        if self.current + 1 >= self.source.len() {
            '\0'
        } else {
            self.source[self.current + 1]
        }
    }

    fn match_string(&mut self) -> Result<(), LoxError> {
        while self.peek() != '"' && !self.is_end() {
            if self.peek() == '\n' {
                self.line += 1;
            }
            self.current += 1;
        }
        if self.is_end() {
            return Err(LoxError::new(self.line, "Unterminated string".to_string()));
        }
        self.current += 1; // 最后的'""
                           // 只拿双引号之间的字符
        let value: String = self.source[self.start + 1..self.current - 1]
            .iter()
            .collect();

        self.add_token(TokenType::String, Literal::Str(value));
        Ok(())
    }

    fn match_number(&mut self) -> Result<(), LoxError> {
        while self.peek().is_numeric() {
            self.current += 1;
        }
        if self.peek() == '.' && self.peek_next().is_numeric() {
            // 略过小数点
            self.current += 1;
            // 处理小数
            while self.peek().is_numeric() {
                self.current += 1;
            }
        }
        let num: f64 = self.source[self.start..self.current]
            .iter()
            .collect::<String>()
            .parse()
            .expect("Error: Parse f64");
        self.add_token(TokenType::Number, Literal::Num(num));
        Ok(())
    }
}
