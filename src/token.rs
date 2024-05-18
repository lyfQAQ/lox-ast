#[derive(Debug)]
enum TokenType {
    // Single-character tokens
    LeftParen,
    RightParen,
    LeftBrace,
    RightBrace,
    CommA,
    Dot,
    Minus,
    Pluse,
    Semicolon,
    Slash,
    Star,
    // One or two character tokens
    Bang,
    BangEqual,
    Equal,
    EqualEqual,
    Greater,
    GreaterEqual,
    Less,
    LessEqual,
    // Literals
    Identifier,
    String,
    Number,
    // Keywords
    And,
    Class,
    Else,
    False,
    Fun,
    For,
    If,
    Nil,
    Or,
    Print,
    Return,
    Super,
    This,
    True,
    Var,
    While,
    Eof,
}

enum Literal {
    Num(f64),
    Str(String),
    Nil,
    True,
    False,
}

impl std::fmt::Display for Literal {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Num(n) => write!(f, "\"{}\"", n),
            Self::Str(s) => write!(f, "\"{}\"", s),
            Self::Nil => write!(f, "nil"),
            Self::True => write!(f, "true"),
            Self::False => write!(f, "false"),
        }
    }
}

struct Token {
    kind: TokenType,
    lexeme: String,
    literal: Option<Literal>,
    line: u32,
}

impl Token {
    fn new(kind: TokenType, lexeme: String, literal: Option<Literal>, line: u32) -> Self {
        Self {
            kind,
            lexeme,
            literal,
            line,
        }
    }
}

impl std::fmt::Display for Token {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{:?} {} {}",
            self.kind,
            self.lexeme,
            if let Some(ref literal) = self.literal {
                literal.to_string()
            } else {
                "None".to_string()
            }
        )
    }
}
