use super::types::TokenType;

pub fn lookup_ident(ident: &str) -> TokenType {
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