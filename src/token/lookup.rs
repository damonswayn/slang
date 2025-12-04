use super::types::TokenType;

pub fn lookup_ident(ident: &str) -> TokenType {
    // Reserve lowercase `test` as the statement keyword, but allow the
    // capitalized `Test` identifier to be used as a namespace (e.g.
    // `Test::assertEq(...)`) without conflicting with the keyword.
    if ident == "Test" {
        return TokenType::Ident;
    }

    match ident.to_lowercase().as_str() {
        "let" => TokenType::Let,
        "true" => TokenType::True,
        "false" => TokenType::False,
        "if" => TokenType::If,
        "else" => TokenType::Else,
        "function" => TokenType::Function,
        "fn" => TokenType::Function,
        "return" => TokenType::Return,
        "while" => TokenType::While,
        "for" => TokenType::For,
        "test" => TokenType::Test,
        _ => TokenType::Ident
    }
}

#[cfg(test)]
mod tests {
    use crate::token::{lookup_ident, TokenType};

    #[test]
    fn lookup_ident_recognizes_let() {
        assert_eq!(lookup_ident("let"), TokenType::Let);
    }

    #[test]
    fn lookup_ident_recognizes_ident() {
        assert_eq!(lookup_ident("foo"), TokenType::Ident);
        assert_eq!(lookup_ident("bar"), TokenType::Ident);
    }
}