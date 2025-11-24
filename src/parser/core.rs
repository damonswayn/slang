use std::collections::HashMap;

use crate::ast::{Program, Statement, Expression, Identifier, IntegerLiteral, InfixExpression, LetStatement, ExpressionStatement, IfExpression, BlockStatement};
use crate::ast::nodes::{BooleanLiteral, FloatLiteral};
use crate::lexer::Lexer;
use crate::token::{Token, TokenType};

#[derive(PartialEq, PartialOrd, Debug, Copy, Clone)]
enum Precedence {
    Lowest = 0,
    Equals, // == !=
    LessGreater, // < > <= >=
    Sum,     // + -
    Product, // * / %
}

fn precedence_of(ttype: &TokenType) -> Precedence {
    use TokenType::*;
    match ttype {
        Equal | NotEqual => Precedence::Equals,
        LessThan | GreaterThan | LessEqual | GreaterEqual => Precedence::LessGreater,
        Plus | Minus => Precedence::Sum,
        Mul | Div | Mod => Precedence::Product,
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

        p.register_infix(TokenType::Equal, Parser::parse_infix_expression);
        p.register_infix(TokenType::NotEqual, Parser::parse_infix_expression);
        p.register_infix(TokenType::LessThan, Parser::parse_infix_expression);
        p.register_infix(TokenType::GreaterThan, Parser::parse_infix_expression);
        p.register_infix(TokenType::LessEqual, Parser::parse_infix_expression);
        p.register_infix(TokenType::GreaterEqual, Parser::parse_infix_expression);

        // register infix parsers
        p.register_infix(TokenType::Plus, Parser::parse_infix_expression);
        p.register_infix(TokenType::Minus, Parser::parse_infix_expression);
        p.register_infix(TokenType::Mul, Parser::parse_infix_expression);
        p.register_infix(TokenType::Div, Parser::parse_infix_expression);
        p.register_infix(TokenType::Mod, Parser::parse_infix_expression);

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

    // ---------- Top-level ----------

    pub fn parse_program(&mut self) -> Program {
        let mut program = Program::new();

        while self.cur_token.token_type != TokenType::Eof {
            if let Some(stmt) = self.parse_statement() {
                program.statements.push(stmt);
            }
            self.next_token();
        }

        program
    }

    fn parse_statement(&mut self) -> Option<Statement> {
        match self.cur_token.token_type {
            TokenType::Let => self.parse_let_statement().map(Statement::Let),
            _ => self.parse_expression_statement().map(Statement::Expression),
        }
    }

    fn parse_let_statement(&mut self) -> Option<LetStatement> {
        // cur_token is 'let'
        if !self.expect_peek(TokenType::Ident) {
            return None;
        }

        let name = Identifier {
            value: self.cur_token.literal.clone(),
        };

        if !self.expect_peek(TokenType::Assign) {
            return None;
        }

        // move to start of expression
        self.next_token();

        let value = self.parse_expression(Precedence::Lowest)?;

        // optional semicolon
        if self.peek_token.token_type == TokenType::Semicolon {
            self.next_token();
        }

        Some(LetStatement { name, value })
    }

    fn parse_expression_statement(&mut self) -> Option<ExpressionStatement> {
        let expr = self.parse_expression(Precedence::Lowest)?;
        if self.peek_token.token_type == TokenType::Semicolon {
            self.next_token();
        }
        Some(ExpressionStatement { expression: expr })
    }

    // ---------- Expressions (Pratt) ----------

    fn parse_expression(&mut self, precedence: Precedence) -> Option<Expression> {
        let prefix = {
            let ttype = self.cur_token.token_type.clone();
            self.prefix_fns.get(&ttype).copied()
        };

        if prefix.is_none() {
            self.no_prefix_parse_fn_error(self.cur_token.token_type.clone());
            return None;
        }

        let mut left = prefix.unwrap()(self)?;

        while self.peek_token.token_type != TokenType::Semicolon
            && precedence < self.peek_precedence()
        {
            let infix = {
                let ttype = self.peek_token.token_type.clone();
                self.infix_fns.get(&ttype).copied()
            };

            if infix.is_none() {
                return Some(left);
            }

            self.next_token(); // advance to operator
            left = infix.unwrap()(self, left)?;
        }

        Some(left)
    }

    fn parse_identifier(&mut self) -> Option<Expression> {
        Some(Expression::Identifier(Identifier {
            value: self.cur_token.literal.clone(),
        }))
    }

    fn parse_integer_literal(&mut self) -> Option<Expression> {
        match self.cur_token.literal.parse::<i64>() {
            Ok(v) => Some(Expression::IntegerLiteral(IntegerLiteral { value: v })),
            Err(_) => {
                self.errors
                    .push(format!("could not parse {} as integer", self.cur_token.literal));
                None
            }
        }
    }

    fn parse_float_literal(&mut self) -> Option<Expression> {
        match self.cur_token.literal.parse::<f64>() {
            Ok(v) => Some(Expression::FloatLiteral(FloatLiteral { value: v })),
            Err(_) => {
                self.errors
                    .push(format!("could not parse {} as float", self.cur_token.literal));
                None
            }
        }
    }

    fn parse_grouped_expression(&mut self) -> Option<Expression> {
        // current is '('
        self.next_token(); // move to first token inside
        let exp = self.parse_expression(Precedence::Lowest)?;
        if !self.expect_peek(TokenType::Rparen) {
            return None;
        }
        Some(exp)
    }

    fn parse_infix_expression(&mut self, left: Expression) -> Option<Expression> {
        let operator = self.cur_token.literal.clone();
        let precedence = self.cur_precedence();
        self.next_token();
        let right = self.parse_expression(precedence)?;
        Some(Expression::Infix(InfixExpression {
            left: Box::new(left),
            operator,
            right: Box::new(right),
        }))
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

    fn no_prefix_parse_fn_error(&mut self, t: TokenType) {
        let msg = format!("no prefix parse function for {:?} found", t);
        self.errors.push(msg);
    }

    fn parse_boolean_literal(&mut self) -> Option<Expression> {
        let value = matches!(self.cur_token.token_type, TokenType::True);
        Some(Expression::BooleanLiteral(BooleanLiteral { value }))
    }

    fn parse_if_expression(&mut self) -> Option<Expression> {
        // current token is 'if'
        if !self.expect_peek(TokenType::Lparen) {
            return None;
        }

        self.next_token(); // move to first token inside '('
        let condition = self.parse_expression(Precedence::Lowest)?;
        if !self.expect_peek(TokenType::Rparen) {
            return None;
        }

        if !self.expect_peek(TokenType::Lbrace) {
            return None;
        }

        let consequence = self.parse_block_statement()?;

        let alternative = if self.peek_token.token_type == TokenType::Else {
            self.next_token(); // current = 'else'
            if !self.expect_peek(TokenType::Lbrace) {
                return None;
            }
            Some(self.parse_block_statement()?)
        } else {
            None
        };

        Some(Expression::If(Box::new(IfExpression {
            condition: Box::new(condition),
            consequence,
            alternative,
        })))
    }

    fn parse_block_statement(&mut self) -> Option<BlockStatement> {
        // current token is '{'
        let mut block = BlockStatement { statements: Vec::new() };

        self.next_token(); // move to first token inside block

        while self.cur_token.token_type != TokenType::Rbrace
            && self.cur_token.token_type != TokenType::Eof
        {
            if let Some(stmt) = self.parse_statement() {
                block.statements.push(stmt);
            }
            self.next_token();
        }

        Some(block)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::lexer::Lexer;

    fn check_errors(p: &Parser) {
        if !p.errors.is_empty() {
            panic!("parser had errors: {:?}", p.errors);
        }
    }

    #[test]
    fn test_let_statements() {
        let input = r#"
        let x = 5;
        let y = 10;
        let foobar = 838383;
        "#;

        let l = Lexer::new(input);
        let mut p = Parser::new(l);
        let program = p.parse_program();
        check_errors(&p);

        assert_eq!(program.statements.len(), 3);

        let names = vec!["x", "y", "foobar"];

        for (i, name) in names.iter().enumerate() {
            match &program.statements[i] {
                Statement::Let(ls) => assert_eq!(ls.name.value, *name),
                _ => panic!("statement {} is not a LetStatement", i),
            }
        }
    }

    #[test]
    fn test_operator_precedence_parsing() {
        let tests = vec![
            ("1 + 2 * 3;", "(1 + (2 * 3))"),
            ("1 * 2 + 3;", "((1 * 2) + 3)"),
            ("(1 + 2) * 3;", "((1 + 2) * 3)"),
        ];

        for (input, expected) in tests {
            let l = Lexer::new(input);
            let mut p = Parser::new(l);
            let program = p.parse_program();
            check_errors(&p);

            let actual = program.to_string();
            assert_eq!(actual, expected);
        }
    }
}

