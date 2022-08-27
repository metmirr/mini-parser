/// The Token enum holds the known token variants
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum Token {
    Plus,
    Dash,
    Star,
    Slash,
    LeftParen,
    RightParen,
    Number(u64), // We only works with unsigned 64 integers
    End,
}

impl Token {
    /// Check if we have leafs or not
    pub fn is_binary(&self) -> bool {
        matches!(self, Token::Plus | Token::Dash | Token::Star | Token::Slash)
    }
}
