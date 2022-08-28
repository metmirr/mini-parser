use crate::{parser::Parser, syntax_error::SyntaxError, tokenizer::tokenizer};
use std::{env, error};

pub mod parser;
pub mod syntax_error;
pub mod token;
pub mod tokenizer;

fn run(input: String) -> Result<u64, SyntaxError> {
    let tokens = tokenizer(input)?;
    let mut token_iter = tokens.iter().peekable();
    let mut parser = Parser::new(&mut token_iter);
    let result = parser.parse();

    match result {
        Ok(mut ast) => Ok(ast.eval()),
        Err(err) => Err(err),
    }
}

fn main() -> Result<(), Box<dyn error::Error>> {
    let args = env::args().collect::<Vec<String>>();
    if args.len() != 2 {
        panic!("Not enough argument!");
    }

    let result = run(args[1].clone()).unwrap();
    println!("Result: {}", result);

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn assert_parsing_success() {
        let actual = run("3a2c4".to_string()).unwrap();
        assert_eq!(actual, 20);

        let actual = run("32a2d2".to_string()).unwrap();
        assert_eq!(actual, 17);

        let actual = run("500a10b66c32".to_string()).unwrap();
        assert_eq!(actual, 14208);

        let actual = run("3ae4c66fb32".to_string()).unwrap();
        assert_eq!(actual, 235);

        let actual = run("3c4d2aee2a4c41fc4f".to_string()).unwrap();
        assert_eq!(actual, 990);
    }
}
