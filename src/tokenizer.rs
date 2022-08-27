///
/// This module is responsible for creating tokens from an input string by using predefined rules:
/// Rules:
/// a = '+'
/// b = '-'
/// c = '*'
/// d = '/'
/// e = '('
/// f = ')
///
use crate::token::Token;
use crate::SyntaxError;

/// Creates an infrinite loop to convert characters to tokens and return a vector of tokens,
///  if it encounter unrecognized character it throws syntax(grammer) error.
/// # Examples
/// ```
/// use tokenizer::tokenizer;
/// let tokens = tokenizer("3a2".to_string());
/// // The tokens would be Ok([Number(3), Plus, Number(2), End])
pub fn tokenizer(input: String) -> Result<Vec<Token>, SyntaxError> {
    // Create a non-consumable iterator
    let mut iter = input.trim().chars().peekable();
    let mut tokens: Vec<Token> = vec![];

    // We will store the last char we have visited, so we know where to continue from
    let mut next_char: Option<char> = None;

    loop {
        let character = match next_char {
            Some(c) => c,
            None => match iter.next() {
                Some(c) => c,
                None => break,
            },
        };
        // We are good so continue
        next_char = None;

        match character {
            'a' => tokens.push(Token::Plus),
            'b' => tokens.push(Token::Dash),
            'c' => tokens.push(Token::Star),
            'd' => tokens.push(Token::Slash),
            'e' => tokens.push(Token::LeftParen),
            'f' => tokens.push(Token::RightParen),
            ch if ch.is_ascii_digit() => {
                // when ch is a digit, we collect the chars till we encounter a rule/token
                let number_stream = iter
                    .by_ref()
                    .take_while(|c| match c.is_ascii_digit() {
                        true => true,
                        false => {
                            next_char = Some(*c);
                            false
                        }
                    })
                    .collect::<String>();
                // We concat 'character' with the 'number_stream' which is the part of the number
                let number = format!("{}{}", character, number_stream)
                    .parse::<u64>()
                    .unwrap();
                tokens.push(Token::Number(number));
            }
            _ => {
                return Err(SyntaxError::tokenizer_error(format!(
                    "Unrecognized character: {}",
                    character
                )));
            }
        }
    }
    // Lets mark we are at the end
    tokens.push(Token::End);

    Ok(tokens)
}

#[cfg(test)]
mod tests {
    use crate::{syntax_error::SyntaxError, tokenizer};

    #[test]
    fn assert_tokenizer_error() {
        let actual = tokenizer("k22".to_string()).err();
        let expected = Some(SyntaxError::tokenizer_error(
            "Unrecognized character: k".to_string(),
        ));
        assert_eq!(actual, expected);
    }
}
