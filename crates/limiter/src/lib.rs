pub mod exercise;
mod lexer;
pub mod parser;
mod syntax;

use exercise::Exercise;
use regex::Regex;

// TODO: The language is NOT context free, so we need to switch to a different kind of interpreter, like a tree-walker
pub struct Parser {}

type ParserResult = Result<exercise::Exercise, String>;

impl Parser {
    pub fn new() -> Self {
        Parser { }
    }

    pub fn parse(exercise_input: String) -> ParserResult {
        
        let re = Regex::new(r"(?<sets>\d+)x(?<reps>\d+)").unwrap();

        let caps = match re.captures(&exercise_input) {
            Some(e) => e,
            None => {
                return ParserResult::Err("Could not find groups".to_string());
            }
        };

        let sets = caps.name("sets").unwrap().as_str().parse::<u64>().unwrap();
        let reps = caps.name("reps").unwrap().as_str().parse::<u64>().unwrap();
        let exercise = Exercise::new(sets, reps);

        Ok(exercise)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_basic_sets_and_reps() {
        assert_eq!(Ok(Exercise::new(3, 10)), Parser::parse("3x10".to_string()));
        // Spaces before and after are ok
        assert_eq!(Ok(Exercise::new(3, 10)), Parser::parse(" 3x10".to_string()));
        assert_eq!(Ok(Exercise::new(3, 10)), Parser::parse(" 3x10 ".to_string()));
    }

    #[test]
    fn parse_invalid_basic_sets_and_reps() {
        assert_eq!(Err("Could not find groups".to_string()), Parser::parse("hello".to_string()));
        assert_eq!(Err("Could not find groups".to_string()), Parser::parse("3x".to_string()));
        assert_eq!(Err("Could not find groups".to_string()), Parser::parse("3".to_string()));
        assert_eq!(Err("Could not find groups".to_string()), Parser::parse("3xA".to_string()));
        // Spaces are not ok in-between
        assert_eq!(Err("Could not find groups".to_string()), Parser::parse("3 x 10".to_string()));
    }
}
