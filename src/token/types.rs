#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum TokenType {
    Illegal,
    Eof,

    Ident,
    Int,
    Float,

    Assign,
    Plus,
    Minus,
    Mul,
    Div,
    Mod,

    Bang,
    LessThan,
    GreaterThan,
    LessEqual,
    GreaterEqual,
    Equal,
    NotEqual,

    Lparen, Rparen,
    Semicolon,

    Let,
    True, False
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Token {
    pub token_type: TokenType,
    pub literal: String,
}

impl Token {
    pub fn new(token_type: TokenType, literal: String) -> Token {
        Token { token_type, literal }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_token_new() {
        let token = Token::new(TokenType::Illegal, String::from(""));
        assert_eq!(token.token_type, TokenType::Illegal);
    }

    #[test]
    fn test_token_literal() {
        let token = Token::new(TokenType::Plus, String::from("+"));
        assert_eq!(token.literal, "+");
        assert_eq!(token.token_type, TokenType::Plus);
    }
}