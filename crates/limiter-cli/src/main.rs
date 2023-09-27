// Goal: Write a program that parses a simple statement in workout notation,
//      then asks the user to record their data for the workout
// Ex
// 3x10
// The program then asks for results in the format a/b/c
use limiter::parser::Parser;
use std::io::{self, Write, Stdout};

fn main() -> io::Result<()> {
    let stdin = io::stdin();
    let mut stdout = io::stdout();

    let mut input = String::new();

    loop {
        write!(stdout, "-> ")?;
        stdout.flush()?;

        stdin.read_line(&mut input)?;

        let parse = Parser::new(&input).parse();

        input.clear();
    }
}
