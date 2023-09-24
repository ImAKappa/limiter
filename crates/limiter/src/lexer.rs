use logos::Logos;

#[derive(Debug, Copy, Clone, PartialEq, Logos)]
pub(crate) enum SyntaxKind {
    #[regex(" +")]
    Whitespace,

    // Literals
    #[regex("[A-Za-z][A-Za-z0-9]*")]
    Ident,

    #[regex(r"#w\d+d\d+")]
    Schedule,

    #[regex("[0-9]+x")]
    Set,

    #[regex("[0-9]*")]
    Number,

    // Keywords
    #[token("program")]
    ProgramKw,

    #[token("super")]
    SuperKw,

    #[token("end")]
    EndKw,

    // Single-Character Tokens
    #[token("+")]
    Plus,

    #[token("-")]
    Minus,

    #[token("/")]
    Slash,

    #[token(":")]
    Colon,

    #[token("(")]
    LParen,

    #[token(")")]
    RParen,
}

#[cfg(test)]
mod tests {
    use super::*;

    fn check(input: &str, kind: SyntaxKind) {
        let mut lexer = SyntaxKind::lexer(input);

        assert_eq!(lexer.next(), Some(Ok(kind)));
        assert_eq!(lexer.slice(), input);
    }

    #[test]
    fn lex_whitespace() {
        check("   ", SyntaxKind::Whitespace);
    }

    #[test]
    fn lex_alphabetic_identifier() {
        check("abcd", SyntaxKind::Ident);
    }

    #[test]
    fn lex_alphanumeric_identifier() {
        check("ab123cde456", SyntaxKind::Ident);
    }

    #[test]
    fn lex_mixed_case_identifier() {
        check("ABCdef", SyntaxKind::Ident);
    }

    #[test]
    fn lex_single_letter_identifier() {
        check("a", SyntaxKind::Ident);
    }

    #[test]
    fn lex_schedule() {
        check("#w1d1", SyntaxKind::Schedule);
        check("#w123d4536", SyntaxKind::Schedule);
    }

    #[test]
    fn lex_set() {
        check("3x", SyntaxKind::Set);
        check("3000x", SyntaxKind::Set);
    }

    #[test]
    fn lex_number() {
        check("123456", SyntaxKind::Number);
    }

    #[test]
    fn lex_program_keyword() {
        check("program", SyntaxKind::ProgramKw);
    }

    #[test]
    fn lex_super_keyword() {
        check("super", SyntaxKind::SuperKw);
    }

    #[test]
    fn lex_end_keyword() {
        check("end", SyntaxKind::EndKw);
    }

    #[test]
    fn lex_plus() {
        check("+", SyntaxKind::Plus);
    }

    #[test]
    fn lex_minus() {
        check("-", SyntaxKind::Minus);
    }

    #[test]
    fn lex_slash() {
        check("/", SyntaxKind::Slash);
    }

    #[test]
    fn lex_colon() {
        check(":", SyntaxKind::Colon);
    }

    #[test]
    fn lex_left_parenthesis() {
        check("(", SyntaxKind::LParen);
    }

    #[test]
    fn lex_right_parenthesis() {
        check(")", SyntaxKind::RParen);
    }
}