use std::collections::HashMap;

use crate::ast::{Program, Expression};
use crate::debug_log;
use crate::lexer::Lexer;
use crate::token::{Token, TokenType};

#[derive(PartialEq, PartialOrd, Debug, Copy, Clone)]
enum Precedence {
    Lowest = 0,
    Assign, // =
    Or, // ||
    And, // &&
    Equals, // == !=
    LessGreater, // < > <= >=
    Sum,     // + -
    Product, // * / %
    Prefix, // !x, -x
    Call, // myFunction(x)
}

fn precedence_of(ttype: &TokenType) -> Precedence {
    use crate::token::TokenType::{
        And, Assign, Div, Equal, GreaterEqual, GreaterThan, Lbracket, LessEqual, LessThan, Mod, Mul,
        NotEqual, Or, Plus, Minus, Lparen,
    };
    match ttype {
        Assign => Precedence::Assign,
        Or => Precedence::Or,
        And => Precedence::And,
        Equal | NotEqual => Precedence::Equals,
        LessThan | GreaterThan | LessEqual | GreaterEqual => Precedence::LessGreater,
        Plus | Minus => Precedence::Sum,
        Mul | Div | Mod => Precedence::Product,
        Lparen => Precedence::Call,
        Lbracket => Precedence::Call,
        _ => Precedence::Lowest,
    }
}

type PrefixParseFn = fn(&mut Parser) -> Option<Expression>;
type InfixParseFn = fn(&mut Parser, Expression) -> Option<Expression>;

pub struct Parser {
    l: Lexer,
    pub errors: Vec<String>,

    cur_token: Token,
    peek_token: Token,

    prefix_fns: HashMap<TokenType, PrefixParseFn>,
    infix_fns: HashMap<TokenType, InfixParseFn>,
}

impl Parser {
    pub fn new(mut l: Lexer) -> Self {
        let first = l.next_token();
        let second = l.next_token();

        let mut p = Parser {
            l,
            errors: Vec::new(),
            cur_token: first,
            peek_token: second,
            prefix_fns: HashMap::new(),
            infix_fns: HashMap::new(),
        };

        // register prefix parsers
        p.register_prefix(TokenType::Ident, Parser::parse_identifier);
        p.register_prefix(TokenType::Int, Parser::parse_integer_literal);
        p.register_prefix(TokenType::Float, Parser::parse_float_literal);
        p.register_prefix(TokenType::Lparen, Parser::parse_grouped_expression);
        p.register_prefix(TokenType::True, Parser::parse_boolean_literal);
        p.register_prefix(TokenType::False, Parser::parse_boolean_literal);
        p.register_prefix(TokenType::If, Parser::parse_if_expression);
        p.register_prefix(TokenType::Bang, Parser::parse_prefix_expression);
        p.register_prefix(TokenType::Minus, Parser::parse_prefix_expression);
        p.register_prefix(TokenType::Function, Parser::parse_function_literal);
        p.register_prefix(TokenType::String, Parser::parse_string_literal);
        p.register_prefix(TokenType::Lbracket, Parser::parse_array_literal);

        // register infix parsers
        p.register_infix(TokenType::Equal, Parser::parse_infix_expression);
        p.register_infix(TokenType::NotEqual, Parser::parse_infix_expression);
        p.register_infix(TokenType::LessThan, Parser::parse_infix_expression);
        p.register_infix(TokenType::GreaterThan, Parser::parse_infix_expression);
        p.register_infix(TokenType::LessEqual, Parser::parse_infix_expression);
        p.register_infix(TokenType::GreaterEqual, Parser::parse_infix_expression);
        p.register_infix(TokenType::And, Parser::parse_infix_expression);
        p.register_infix(TokenType::Or, Parser::parse_infix_expression);
        p.register_infix(TokenType::Assign, Parser::parse_infix_expression);

        p.register_infix(TokenType::Plus, Parser::parse_infix_expression);
        p.register_infix(TokenType::Minus, Parser::parse_infix_expression);
        p.register_infix(TokenType::Mul, Parser::parse_infix_expression);
        p.register_infix(TokenType::Div, Parser::parse_infix_expression);
        p.register_infix(TokenType::Mod, Parser::parse_infix_expression);

        p.register_infix(TokenType::Lparen, Parser::parse_call_expression);
        p.register_infix(TokenType::Lbracket, Parser::parse_index_expression);

        p
    }

    fn next_token(&mut self) {
        self.cur_token = self.peek_token.clone();
        self.peek_token = self.l.next_token();
    }

    fn register_prefix(&mut self, ttype: TokenType, func: PrefixParseFn) {
        self.prefix_fns.insert(ttype, func);
    }

    fn register_infix(&mut self, ttype: TokenType, func: InfixParseFn) {
        self.infix_fns.insert(ttype, func);
    }

    pub fn parse_program(&mut self) -> Program {
        let mut program = Program { statements: Vec::new() };

        debug_log!("parse_program: starting, cur_token = {:?}", self.cur_token);

        while self.cur_token.token_type != TokenType::Eof {
            debug_log!("parse_program: top of loop, cur_token = {:?}", self.cur_token);

            match self.parse_statement() {
                Some(stmt) => {
                    debug_log!("  parse_statement returned: {:?}", stmt);
                    program.statements.push(stmt);
                }
                None => {
                    debug_log!("  parse_statement returned None");
                }
            }

            self.next_token();
        }

        debug_log!(
            "parse_program: finished with {} statements",
            program.statements.len()
        );

        program
    }

    fn peek_precedence(&self) -> Precedence {
        precedence_of(&self.peek_token.token_type)
    }

    fn cur_precedence(&self) -> Precedence {
        precedence_of(&self.cur_token.token_type)
    }

    // ---------- Helpers ----------

    fn expect_peek(&mut self, ttype: TokenType) -> bool {
        if self.peek_token.token_type == ttype {
            self.next_token();
            true
        } else {
            self.peek_error(ttype);
            false
        }
    }

    fn peek_error(&mut self, ttype: TokenType) {
        let msg = format!(
            "expected next token to be {:?}, got {:?} instead",
            ttype, self.peek_token.token_type
        );
        self.errors.push(msg);
    }
}

mod expr;
mod stmt;

#[cfg(test)]
mod tests;

