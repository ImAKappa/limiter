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

    #[token("week")]
    WeekKw,

    #[token("day")]
    DayKw,

    #[token("ex")]
    ExKw,

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

    #[token("[")]
    LSqBracket,

    #[token("]")]
    RSqBracket,

    #[token(",")]
    Comma,

    #[token(";")]
    Semicolon,

    #[token("#")]
    Pound,

    #[token("@")]
    At,
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

    // Literals
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
    fn lex_set() {
        check("3x", SyntaxKind::Set);
        check("3000x", SyntaxKind::Set);
    }

    #[test]
    fn lex_number() {
        check("123456", SyntaxKind::Number);
    }

    // Keywords

    #[test]
    fn lex_program_keyword() {
        check("program", SyntaxKind::ProgramKw);
    }

    #[test]
    fn lex_super_keyword() {
        check("super", SyntaxKind::SuperKw);
    }

    #[test]
    fn lex_week_keyword() {
        check("week", SyntaxKind::WeekKw);
    }

    #[test]
    fn lex_day_keyword() {
        check("day", SyntaxKind::DayKw);
    }

    #[test]
    fn lex_exercise_keyword() {
        check("ex", SyntaxKind::ExKw);
    }

    #[test]
    fn lex_end_keyword() {
        check("end", SyntaxKind::EndKw);
    }

    // Single-Character Tokens

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

    #[test]
    fn lex_left_square_bracket() {
        check("[", SyntaxKind::LSqBracket);
    }

    #[test]
    fn lex_right_square_bracket() {
        check("]", SyntaxKind::RSqBracket);
    }

    #[test]
    fn lex_comma() {
        check(",", SyntaxKind::Comma);
    }

    #[test]
    fn lex_semicolon() {
        check(";", SyntaxKind::Semicolon);
    }

    #[test]
    fn lex_pound() {
        check("#", SyntaxKind::Pound);
    }

    #[test]
    fn lex_at_sign() {
        check("@", SyntaxKind::At);
    }
}