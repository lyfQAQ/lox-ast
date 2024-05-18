use crate::token::*;

struct Scanner {
    source: String,
    tokens: Vec<Token>,
    // the first character in the lexeme being scanned
    start: u32,
    // the character currently being considered
    current: u32,
    line: u32,
}

impl Scanner {
    fn new(source: String) -> Self {
        Self {
            source: source,
            tokens: Vec::new(),
            start: 0,
            current: 0,
            line: 0,
        }
    }

    fn scan_tokens(&mut self) {
        while !is_end() {
            self.start = self.current;
            self.scan_token();
        }

        // add "end of file" token
        self.tokens.push(Token::new(
            TokenType::Eof,
            String::new(),
            Some(Literal::Nil),
            line,
        ));
    }

    fn is_end(&self) -> bool {
        self.current >= self.source.len() as u32
    }
}
