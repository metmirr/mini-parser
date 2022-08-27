use crate::token::Token;
use crate::SyntaxError;
use std::{iter, slice};

/// The Operator enum holds the known operators
#[derive(Debug)]
pub enum Operator {
    Add,
    Multiply,
    Divide,
    Substract,
}

impl Operator {
    // Try to match token with operator if token is not an operator then fail
    fn get_operator_or_error(token: &Token) -> Result<Operator, &'static str> {
        match token {
            Token::Plus => Ok(Operator::Add),
            Token::Dash => Ok(Operator::Substract),
            Token::Star => Ok(Operator::Multiply),
            Token::Slash => Ok(Operator::Divide),
            _ => Err("Can match only operators"),
        }
    }

    // Help us to determine inner precedence when there are parantheses.
    // Here all the arms have the same value since we dont' care the order of operators except parantheses.
    fn cmp_val(&self) -> u8 {
        match self {
            Operator::Add => 3,
            Operator::Substract => 3,
            Operator::Multiply => 3,
            Operator::Divide => 3,
        }
    }
}

/// The Expression enum holds possible expressions we are about
#[derive(Debug)]
pub enum Expression {
    Binary(Operator, Box<Expression>, Box<Expression>),
    Number(u64),
}

impl Expression {
    /// Evaluate the generated AST
    pub fn eval(&mut self) -> u64 {
        match self {
            Expression::Number(n) => *n,
            Expression::Binary(Operator::Add, expr1, expr2) => expr1.eval() + expr2.eval(),
            Expression::Binary(Operator::Substract, expr1, expr2) => expr1.eval() - expr2.eval(),
            Expression::Binary(Operator::Multiply, expr1, expr2) => expr1.eval() * expr2.eval(),
            Expression::Binary(Operator::Divide, expr1, expr2) => expr1.eval() / expr2.eval(),
        }
    }
}

/// The parser struct take a peekable token iterator and convert tokens into expression to make AST
pub struct Parser<'a> {
    pub iter: &'a mut iter::Peekable<slice::Iter<'a, Token>>,
}

impl<'a> Parser<'a> {
    /// Create a new parser with the provided token iterator
    pub fn new(iter: &'a mut iter::Peekable<slice::Iter<'a, Token>>) -> Self {
        Self { iter }
    }

    /// Check next token to match provided token otherwise return different error messages for each case
    /// first case: for such an input '1c': "Unexpected end of input"
    /// second case: for such an input '1c(3a1': "Expected RightParen got End"
    pub fn check_next_to_be(&mut self, token: Token) -> Result<(), SyntaxError> {
        let next = self.iter.next();
        if next.is_none() {
            return Err(SyntaxError::parse_error(
                "Unexpected end of input".to_string(),
            ));
        }

        let next_val = next.unwrap();
        if *next_val != token {
            return Err(SyntaxError::parse_error(format!(
                "Expected {:?} got {:?}",
                token, next_val
            )));
        }
        Ok(())
    }

    /// Check if starting is with not operators but number or parantheses:
    /// if parantheses then follow the expression func
    pub fn primary(&mut self) -> Result<Expression, SyntaxError> {
        match self.iter.next().unwrap() {
            Token::LeftParen => {
                let expr = self.expression(0)?;
                // Eventually we should close the parantheses
                self.check_next_to_be(Token::RightParen)?;
                Ok(expr)
            }
            Token::Number(n) => Ok(Expression::Number(*n)),
            token => Err(SyntaxError::parse_error(format!(
                "Unexpected token found {:?}",
                token
            ))),
        }
    }

    /// Create AST from tokens
    pub fn expression(&mut self, precedence: u8) -> Result<Expression, SyntaxError> {
        let mut expr = self.primary()?;
        while let Some(token) = self.iter.peek() {
            // if it is not binary token(operator) then go back where we left
            if !token.is_binary() {
                break;
            }
            // get the corresponding operator for the token
            let operator = Operator::get_operator_or_error(*token).unwrap();

            // Take care of the order of operator for being inner(in the parantheses)
            if operator.cmp_val() < precedence {
                break;
            }
            // Advance iterator for next token
            self.iter.next();
            // increase precedence hence we are in parantheses
            let inner_precedence = operator.cmp_val() + 1;

            let right_hand_side = self.expression(inner_precedence)?;
            // Create expression from left(expr) and right(right_hand_side)
            expr = Expression::Binary(operator, Box::new(expr), Box::new(right_hand_side));
        }

        Ok(expr)
    }

    pub fn parse(&mut self) -> Result<Expression, SyntaxError> {
        let ast = self.expression(0)?;
        // expect to be end of the expression
        self.check_next_to_be(Token::End)?;
        Ok(ast)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn assert_parse_error() {
        let tokens = vec![Token::End];
        let mut tokens_iter = tokens.iter().peekable();
        let mut parser = Parser::new(&mut tokens_iter);

        let actual = parser.parse().err();
        let expected = Some(SyntaxError::parse_error(
            "Unexpected token found End".to_string(),
        ));

        assert_eq!(actual, expected);
    }
}
