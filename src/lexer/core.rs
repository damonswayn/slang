use crate::token::{lookup_ident, Token, TokenType};

pub struct Lexer {
    input: Vec<char>,
    position: usize,
    read_position: usize,
    ch: Option<char>,
}

impl Lexer {
    pub fn new(input: &str) -> Self {
        let mut l = Lexer {
            input: input.chars().collect(),
            position: 0,
            read_position: 0,
            ch: None
        };

        l.read_char();
        l
    }

    pub fn read_char(&mut self) {
        if self.read_position >= self.input.len() {
            self.ch = None;
        } else {
            self.ch = Some(self.input[self.read_position]);
        }

        self.position = self.read_position;
        self.read_position += 1;
    }

    pub fn next_token(&mut self) -> Token {
        self.skip_whitespace();

        let tok = match self.ch {
            Some('=') => {
                if self.peek_char() == Some('=') {
                    self.read_char();
                    Token::new(TokenType::Equal, String::from("=="))
                } else {
                    Token::new(TokenType::Assign, String::from("="))
                }
            },
            Some('!') => {
                if self.peek_char() == Some('=') {
                    self.read_char();
                    Token::new(TokenType::NotEqual, String::from("!="))
                } else {
                    Token::new(TokenType::Bang, String::from("!"))
                }
            },
            Some('<') => {
                if self.peek_char() == Some('=') {
                    self.read_char();
                    Token::new(TokenType::LessEqual, String::from("<="))
                } else {
                    Token::new(TokenType::LessThan, String::from("<"))
                }
            },
            Some('>') => {
                if self.peek_char() == Some('=') {
                    self.read_char();
                    Token::new(TokenType::GreaterEqual, String::from(">="))
                } else {
                    Token::new(TokenType::GreaterThan, String::from(">"))
                }
            },
            Some('&') => {
                if self.peek_char() == Some('&') {
                    self.read_char();
                    Token::new(TokenType::And, String::from("&&"))
                } else {
                    Token::new(TokenType::Illegal, String::from("&"))
                }
            },
            Some('|') => {
                if self.peek_char() == Some('|') {
                    self.read_char();
                    Token::new(TokenType::Or, String::from("||"))
                } else {
                    Token::new(TokenType::Illegal, String::from("|"))
                }
            },
            Some('"') => {
                let literal = self.read_string();
                Token::new(TokenType::String, literal)
            },
            Some('+') => Token::new(TokenType::Plus, String::from("+")),
            Some('-') => Token::new(TokenType::Minus, String::from("-")),
            Some('*') => Token::new(TokenType::Mul, String::from("*")),
            Some('/') => Token::new(TokenType::Div, String::from("/")),
            Some('%') => Token::new(TokenType::Mod, String::from("%")),
            Some('(') => Token::new(TokenType::Lparen, String::from("(")),
            Some(')') => Token::new(TokenType::Rparen, String::from(")")),
            Some('{') => Token::new(TokenType::Lbrace, String::from("{")),
            Some('}') => Token::new(TokenType::Rbrace, String::from("}")),
            Some(';') => Token::new(TokenType::Semicolon, String::from(";")),
            Some(',') => Token::new(TokenType::Comma, String::from(",")),
            Some('[' ) => Token::new(TokenType::Lbracket, String::from("[")),
            Some(']') => Token::new(TokenType::Rbracket, String::from("]")),
            None => Token::new(TokenType::Eof, String::from("")),
            Some(ch) => {
                if is_letter(ch) {
                    let literal = self.read_identifier();
                    let ttype = lookup_ident(&literal);
                    return Token::new(ttype, literal);
                } else if ch.is_ascii_digit() {
                    let (literal, is_float) = self.read_number();
                    let ttype = if is_float { TokenType::Float } else { TokenType::Int };
                    return Token::new(ttype, literal);
                } else {
                    Token::new(TokenType::Illegal, String::from(ch))
                }
            }
        };

        self.read_char();
        tok
    }

    pub fn skip_whitespace(&mut self) {
        while matches!(self.ch, Some(' ') | Some('\t') | Some('\n') | Some('\r')) {
            self.read_char();
        }
    }

    fn read_identifier(&mut self) -> String {
        let start = self.position;

        while matches!(self.ch, Some(ch) if is_letter(ch) || ch.is_ascii_digit()) {
            self.read_char();
        }

        self.input[start..self.position].iter().collect()
    }

    fn read_number(&mut self) -> (String, bool) {
        let start = self.position;

        while matches!(self.ch, Some(ch) if ch.is_ascii_digit()) {
            self.read_char();
        }

        let mut is_float = false;
        if self.ch == Some('.') {
            if let Some(next_ch) = self.peek_char() {
                if next_ch.is_ascii_digit() {
                    is_float = true;
                    self.read_char();
                    while matches!(self.ch, Some(ch) if ch.is_ascii_digit()) {
                        self.read_char();
                    }
                }
            }
        }

        let literal: String = self.input[start..self.position].iter().collect();
        (literal, is_float)
    }

    fn peek_char(&self) -> Option<char> {
        if self.read_position >= self.input.len() {
            None
        } else {
            Some(self.input[self.read_position])
        }
    }

    fn read_string(&mut self) -> String {
        // currently self.ch == '"'
        self.read_char();            // move to first char after the quote
        let start = self.position;

        while self.ch != Some('"') && self.ch != Some('\0') {
            self.read_char();
        }

        // at this point self.ch == '"' or '\0'
        let s = self.input[start..self.position].iter().collect();

        // DO NOT call read_char() here
        s
    }
}

fn is_letter(ch: char) -> bool {
    ch.is_ascii_alphabetic() || ch == '_'
}

#[cfg(test)]
mod tests {
    use super::Lexer;
    use crate::token::TokenType::{Assign, Eof, Ident, Int, Let, Lparen, Mul, Plus, Rparen, Semicolon};

    #[test]
    fn test_next_token() {
        let input = r#"
        let x = 5;
        let y = x + 10;
        (x + y) * 2;
        "#;

        let tests = vec![
            (Let, "let"),
            (Ident, "x"),
            (Assign, "="),
            (Int, "5"),
            (Semicolon, ";"),
            (Let, "let"),
            (Ident, "y"),
            (Assign, "="),
            (Ident, "x"),
            (Plus, "+"),
            (Int, "10"),
            (Semicolon, ";"),
            (Lparen, "("),
            (Ident, "x"),
            (Plus, "+"),
            (Ident, "y"),
            (Rparen, ")"),
            (Mul, "*"),
            (Int, "2"),
            (Semicolon, ";"),
            (Eof, ""),
        ];

        let mut l = Lexer::new(input);

        for (i, (expected_type, expected_literal)) in tests.into_iter().enumerate() {
            let tok = l.next_token();

            assert_eq!(
                tok.token_type, expected_type,
                "tests[{}] - token_type wrong. expected={:?}, got={:?}",
                i, expected_type, tok.token_type
            );
            assert_eq!(
                tok.literal, expected_literal,
                "tests[{}] - literal wrong. expected={}, got={}",
                i, expected_literal, tok.literal
            );
        }
    }
}