use crate::{lexer::{SyntaxKind, Lexer}, syntax::LimiterLanguage};
use logos::Logos;
use rowan::{GreenNode, GreenNodeBuilder, Language};
use std::iter::Peekable;

pub struct Parser<'a> {
    lexer: Peekable<Lexer<'a>>,
    builder: GreenNodeBuilder<'static>,
}

impl<'a> Parser<'a> {
    pub fn new(input: &'a str) -> Self {
        Self {
            lexer: Lexer::new(input).peekable(),
            builder: GreenNodeBuilder::new(),
        }
    }

    pub fn parse(mut self) -> Parse {
        self.start_node(SyntaxKind::Root);

        match self.peek() {
            Some(SyntaxKind::Number) | Some(SyntaxKind::Ident) => self.bump(),
            _ => {},
        }

        self.finish_node();
        
        Parse {
            green_node: self.builder.finish(),
        }
    }

    pub(crate) fn start_node(&mut self, kind: SyntaxKind) {
        self.builder.start_node(LimiterLanguage::kind_to_raw(kind));
    }

    pub(crate) fn finish_node(&mut self) {
        self.builder.finish_node();
    }

    fn bump(&mut self) {
        let (kind, text) = self.lexer.next().unwrap();

        self.builder
            .token(LimiterLanguage::kind_to_raw(kind), text.into());
    }

    fn peek(&mut self) -> Option<SyntaxKind> {
        self.lexer.peek().map(|(kind, _)| *kind)
    }
}

pub struct Parse {
    green_node: GreenNode,
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::syntax::{SyntaxNode, self};
    use expect_test::{expect, Expect};

    fn check(input: &str, expected_tree: Expect) {
        let parse = Parser::new(input).parse();
        let syntax_node = SyntaxNode::new_root(parse.green_node);

        let actual_tree = format!("{:#?}", syntax_node);

        expected_tree.assert_eq(&actual_tree[0..actual_tree.len() - 1]);
    }

    #[test]
    fn parse_nothing() {
        check("", expect![[r#"Root@0..0"#]]);
    }

    #[test]
    fn parse_number() {
        check(
            "123",
            expect![[r#"
Root@0..3
  Number@0..3 "123""#]],
        );
    }

    #[test]
    fn parse_binding_usage() {
        check(
            "counter",
            expect![[r#"
Root@0..7
  Ident@0..7 "counter""#]],
        );
    }
}