pub enum Token {
    // Single-character Tokens
    Cross,
    Plus,
    Slash,
    Minus,
    LParen,
    RParen,
    Comma,
    
    // Literals
    Number {
        scale: u64,
        unit: String,
    },
    Note(String),
}