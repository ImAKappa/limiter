#[derive(Debug)]
pub enum TokenType {
    // Single-character Tokens
    Cross,
    Plus,
    Slash,
    Minus,
    LParen,
    RParen,
    Comma,
    Colon,
    
    // Literals
    RepetitionMax,
    DurationMax,
    Number {
        scale: u64,
        unit: String,
    },
    Identifier,
    Note(String),

    // Keywords
    Program,
    Week,
    Day,
    Super,
    End,

    // End of Input
    EOL,
    EOF,
}

#[derive(Debug)]
pub struct Token {
    pub ttype: TokenType,
    pub text: String,
    pub line: usize,
    pub first_column: usize,
    pub last_column: usize,
}

impl Token {
    fn new(ttype: TokenType, text: String, line: usize, first_column: usize, last_column: usize) -> Self {
        Token {
            ttype,
            text,
            line,
            first_column,
            last_column,
        }
    }
}

#[derive(Debug)]
pub enum Mode {
    Default,
}

#[derive(Debug)]
pub struct State {
    pub stack: Vec<Mode>,
    pub line: usize,
}

fn get_tokens(text: String) -> Vec<Token> {
    let mut tokens = Vec::<Token>::new();
    let mut state = State {stack: vec![Mode::Default], line: 1};

    for line in text.split('\n') {
        let stream = line.chars().peekable();
        while 
    }

    tokens
}