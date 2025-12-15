#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum TokenType {
    Illegal,
    Eof,

    Ident,
    Int,
    Float,
    String,
    Function,

    Assign,
    Plus,
    PlusPlus,
    Minus,
    MinusMinus,
    Mul,
    Div,
    Mod,

    Dot,
    /// Double-colon, used for qualified access like `Option::Some`
    ColonColon,

    And,
    Or,

    Bang,
    LessThan,
    GreaterThan,
    LessEqual,
    GreaterEqual,
    Equal,
    NotEqual,

    Lparen, Rparen,
    Lbrace, Rbrace,
    Lbracket, Rbracket,
    Semicolon,
    Comma,
    Colon,
    Arrow,

    Let,
    True, False,
    If, Else,
    Return,
    While,
    For,
    Test,
    Namespace,
    Import,
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
    use super::{Token, TokenType};

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