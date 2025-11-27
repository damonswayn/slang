use std::collections::HashMap;

use crate::ast::{Program, Statement, Expression, Identifier, IntegerLiteral, InfixExpression, LetStatement, ExpressionStatement, IfExpression, BlockStatement, FunctionLiteral, CallExpression, WhileStatement, StringLiteral, ArrayLiteral, IndexExpression};
use crate::ast::nodes::{BooleanLiteral, FloatLiteral, ForStatement, FunctionStatement, PrefixExpression, ReturnStatement};
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
    use TokenType::*;
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

    fn parse_statement(&mut self) -> Option<Statement> {
        match self.cur_token.token_type {
            TokenType::Let => {
                debug_log!("  -> parsing Let statement");
                self.parse_let_statement().map(Statement::Let)
            },
            TokenType::Return => {
                debug_log!("  -> parsing Return statement");
                self.parse_return_statement().map(Statement::Return)
            },
            TokenType::While => {
                debug_log!("  -> parsing While statement");
                self.parse_while_statement().map(Statement::While)
            },
            TokenType::For => {
                debug_log!("  -> parsing For statement");
                self.parse_for_statement().map(Statement::For)
            },
            TokenType::Function => {
                debug_log!("  -> parsing Function statement");
                self.parse_function_statement().map(Statement::Function)
            },
            _ => {
                debug_log!("  -> default: parsing Expression statement");
                let stmt = self.parse_expression_statement();
                debug_log!("  -> parse_expression_statement returned: {:?}", stmt);
                stmt.map(Statement::Expression)
            },
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
        debug_log!("parse_expression_statement: ENTER, cur_token = {:?}", self.cur_token);

        let expr = match self.parse_expression(Precedence::Lowest) {
            Some(e) => {
                debug_log!("parse_expression_statement: parse_expression returned Some({:?})", e);
                e
            }
            None => {
                debug_log!("parse_expression_statement: parse_expression returned None");
                return None;
            }
        };

        debug_log!(
            "parse_expression_statement: after parse_expression, cur_token = {:?}, peek_token = {:?}",
            self.cur_token, self.peek_token
        );

        if self.peek_token.token_type == TokenType::Semicolon {
            debug_log!("parse_expression_statement: consuming trailing semicolon");
            self.next_token();
        }

        let stmt = ExpressionStatement { expression: expr };
        debug_log!("parse_expression_statement: EXIT with {:?}", stmt);
        Some(stmt)
    }

    // ---------- Expressions (Pratt) ----------

    fn parse_expression(&mut self, precedence: Precedence) -> Option<Expression> {
        debug_log!(
            "parse_expression: ENTER, cur_token = {:?}, peek_token = {:?}, precedence = {:?}",
            self.cur_token, self.peek_token, precedence
        );

        let prefix = match self.prefix_fns.get(&self.cur_token.token_type).copied() {
            Some(prefix) => {
                debug_log!("parse_expression: found prefix fn for {:?}", self.cur_token.token_type);
                prefix
            }
            None => {
                debug_log!(
                    "parse_expression: NO prefix fn for {:?}, returning None",
                    self.cur_token.token_type
                );
                return None;
            }
        };

        let mut left_exp = match prefix(self) {
            Some(e) => {
                debug_log!("parse_expression: prefix parsed to {:?}", e);
                e
            }
            None => {
                debug_log!("parse_expression: prefix parser returned None");
                return None;
            }
        };

        debug_log!(
            "parse_expression: after prefix, cur_token = {:?}, peek_token = {:?}",
            self.cur_token, self.peek_token
        );

        while self.peek_token.token_type != TokenType::Semicolon
            && precedence < self.peek_precedence()
        {
            debug_log!(
                "parse_expression: infix loop, peek_token = {:?}, precedence = {:?}, peek_prec = {:?}",
                self.peek_token,
                precedence,
                self.peek_precedence()
            );

            let infix = match self.infix_fns.get(&self.peek_token.token_type).copied() {
                Some(infix) => {
                    debug_log!(
                        "parse_expression: found infix fn for {:?}",
                        self.peek_token.token_type
                    );
                    infix
                }
                None => {
                    debug_log!(
                        "parse_expression: NO infix fn for {:?}, breaking",
                        self.peek_token.token_type
                    );
                    return Some(left_exp);
                }
            };

            self.next_token(); // step onto the infix operator
            debug_log!(
                "parse_expression: after next_token, cur_token = {:?}, calling infix",
                self.cur_token
            );

            left_exp = match infix(self, left_exp) {
                Some(e) => {
                    debug_log!("parse_expression: infix parsed to {:?}", e);
                    e
                }
                None => {
                    debug_log!("parse_expression: infix parser returned None");
                    return None;
                }
            };

            debug_log!(
                "parse_expression: end of infix loop iteration, cur_token = {:?}, peek_token = {:?}",
                self.cur_token, self.peek_token
            );
        }

        debug_log!("parse_expression: EXIT with {:?}", left_exp);
        Some(left_exp)
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
        self.next_token(); // move to the first token inside
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

    fn parse_prefix_expression(&mut self) -> Option<Expression> {
        let operator = self.cur_token.literal.clone();

        self.next_token(); // move to the right-hand side

        let right = self.parse_expression(Precedence::Prefix)?;

        Some(Expression::Prefix(Box::new(PrefixExpression {
            operator,
            right: Box::new(right),
        })))
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

    fn parse_boolean_literal(&mut self) -> Option<Expression> {
        let value = matches!(self.cur_token.token_type, TokenType::True);
        Some(Expression::BooleanLiteral(BooleanLiteral { value }))
    }

    fn parse_if_expression(&mut self) -> Option<Expression> {
        // the current token is 'if'
        if !self.expect_peek(TokenType::Lparen) {
            return None;
        }

        self.next_token(); // move to the first token inside '('
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

    fn parse_function_literal(&mut self) -> Option<Expression> {
        // current token is 'fn'
        if !self.expect_peek(TokenType::Lparen) {
            return None;
        }

        let params = self.parse_function_parameters()?;

        if !self.expect_peek(TokenType::Lbrace) {
            return None;
        }

        let body = self.parse_block_statement()?;

        Some(Expression::FunctionLiteral(FunctionLiteral { params, body }))
    }

    fn parse_call_expression(&mut self, function: Expression) -> Option<Expression> {
        debug_log!(
            "parse_call_expression: ENTER, function = {:?}, cur_token = {:?}, peek_token = {:?}",
            function, self.cur_token, self.peek_token
        );

        let arguments = self.parse_expression_list(TokenType::Rparen)?;
        debug_log!("parse_call_expression: arguments = {:?}", arguments);

        Some(Expression::CallExpression(Box::new(CallExpression {
            function: Box::new(function),
            arguments,
        })))
    }

    fn parse_function_parameters(&mut self) -> Option<Vec<Identifier>> {
        let mut params = Vec::new();

        // fn() ...
        if self.peek_token.token_type == TokenType::Rparen {
            self.next_token(); // skip ')'
            return Some(params);
        }

        // first param
        self.next_token(); // current = first identifier
        params.push(Identifier { value: self.cur_token.literal.clone() });

        // more params
        while self.peek_token.token_type == TokenType::Comma {
            self.next_token(); // skip ','
            self.next_token(); // move to next ident
            params.push(Identifier { value: self.cur_token.literal.clone() });
        }

        if !self.expect_peek(TokenType::Rparen) {
            return None;
        }

        Some(params)
    }

    fn parse_return_statement(&mut self) -> Option<ReturnStatement> {
        // the current token is 'return'
        self.next_token(); // move to start of expression

        // We allow: return; (no value) which just returns null
        if self.cur_token.token_type == TokenType::Semicolon {
            return Some(ReturnStatement {
                return_value: Expression::IntegerLiteral(IntegerLiteral { value: 0 }), // placeholder if you want, or special-case in evaluator
            });
        }

        let value = self.parse_expression(Precedence::Lowest)?;

        // optional semicolon
        if self.peek_token.token_type == TokenType::Semicolon {
            self.next_token();
        }

        Some(ReturnStatement { return_value: value })
    }

    fn parse_while_statement(&mut self) -> Option<WhileStatement> {
        // the current token is 'while'
        if !self.expect_peek(TokenType::Lparen) {
            return None;
        }

        self.next_token(); // move to the first token inside '('
        let condition = self.parse_expression(Precedence::Lowest)?;
        if !self.expect_peek(TokenType::Rparen) {
            return None;
        }

        if !self.expect_peek(TokenType::Lbrace) {
            return None;
        }

        let body = self.parse_block_statement()?;

        Some(WhileStatement { condition, body })
    }

    fn parse_for_statement(&mut self) -> Option<ForStatement> {
        // the current token is 'for'
        if !self.expect_peek(TokenType::Lparen) {
            return None;
        }
        // cur_token is now '('

        // === INIT ===
        // Move to the first token of init or ';'
        self.next_token();

        let init: Option<Box<Statement>> = if self.cur_token.token_type == TokenType::Semicolon {
            // for (; cond; post)
            None
        } else if self.cur_token.token_type == TokenType::Let {
            // for (let x = 0; ...
            // parse_let_statement will parse up to (and consume) the ';'
            let let_stmt = self.parse_let_statement()?;
            Some(Box::new(Statement::Let(let_stmt)))
        } else {
            // for (expr; ...
            // parse an expression, then explicitly require a ';'
            let expr = self.parse_expression(Precedence::Lowest)?;
            if !self.expect_peek(TokenType::Semicolon) {
                return None;
            }
            let expr_stmt = ExpressionStatement { expression: expr };
            Some(Box::new(Statement::Expression(expr_stmt)))
        };

        // After init, the current token should be the ';' that ends init.
        // Move to the first token of condition or ';'
        if self.cur_token.token_type != TokenType::Semicolon {
            // if parse_let_statement consumed it internally, cur_token might already be after it;
            // but in the usual Monkey-style parser it's sitting on the ';', so we don't hard-panic.
        }
        self.next_token();

        // === CONDITION ===
        let condition: Option<Expression> = if self.cur_token.token_type == TokenType::Semicolon {
            // for (; ; post)  => no condition (treated like "true")
            None
        } else {
            let expr = self.parse_expression(Precedence::Lowest)?;
            // header syntax requires an ';' after the condition
            if !self.expect_peek(TokenType::Semicolon) {
                return None;
            }
            Some(expr)
        };

        // After condition, the current token is the ';' that ends the condition.
        // Move to the first token of post or ')'
        self.next_token();

        // === POST ===
        let post: Option<Box<Statement>> = if self.cur_token.token_type == TokenType::Rparen {
            // for (...; ...; )  => no post
            None
        } else {
            // parse post as a bare expression (no trailing ';' in the header)
            let expr = self.parse_expression(Precedence::Lowest)?;
            let expr_stmt = ExpressionStatement { expression: expr };

            if !self.expect_peek(TokenType::Rparen) {
                return None;
            }

            Some(Box::new(Statement::Expression(expr_stmt)))
        };

        // === BODY ===
        if !self.expect_peek(TokenType::Lbrace) {
            return None;
        }
        let body = self.parse_block_statement()?;

        Some(ForStatement {
            init,
            condition,
            post,
            body,
        })
    }

    fn parse_function_statement(&mut self) -> Option<FunctionStatement> {
        if !self.expect_peek(TokenType::Ident) {
            return None;
        }

        let name = Identifier {
            value: self.cur_token.literal.clone(),
        };

        if !self.expect_peek(TokenType::Lparen) {
            return None;
        }

        let params = self.parse_function_parameters()?;

        if !self.expect_peek(TokenType::Lbrace) {
            return None;
        }

        let body = self.parse_block_statement()?;

        Some(FunctionStatement {
            name,
            literal: FunctionLiteral {
                params,
                body,
            },
        })
    }

    fn parse_string_literal(&mut self) -> Option<Expression> {
        Some(Expression::StringLiteral(StringLiteral {
            value: self.cur_token.literal.clone(),
        }))
    }

    fn parse_array_literal(&mut self) -> Option<Expression> {
        // current token is '['
        let elements = self.parse_expression_list(TokenType::Rbracket)?;
        Some(Expression::ArrayLiteral(ArrayLiteral { elements }))
    }

    fn parse_expression_list(&mut self, end: TokenType) -> Option<Vec<Expression>> {
        debug_log!(
            "parse_expression_list: ENTER, end = {:?}, cur_token = {:?}, peek_token = {:?}",
            end, self.cur_token, self.peek_token
        );

        let mut list = Vec::new();

        if self.peek_token.token_type == end {
            debug_log!("parse_expression_list: empty list (peek == end)");
            self.next_token(); // consume ')'
            return Some(list);
        }

        self.next_token(); // move to first argument
        debug_log!(
            "parse_expression_list: after first next_token, cur_token = {:?}, peek_token = {:?}",
            self.cur_token, self.peek_token
        );

        list.push(self.parse_expression(Precedence::Lowest)?);
        debug_log!("parse_expression_list: after first arg, list = {:?}", list);

        while self.peek_token.token_type == TokenType::Comma {
            debug_log!("parse_expression_list: found comma, parsing another arg");
            self.next_token(); // consume ','
            self.next_token(); // move to next argument
            list.push(self.parse_expression(Precedence::Lowest)?);
            debug_log!("parse_expression_list: list now = {:?}", list);
        }

        if !self.expect_peek(end) {
            debug_log!("parse_expression_list: expect_peek(end) failed");
            return None;
        }

        debug_log!("parse_expression_list: EXIT with {:?}", list);
        Some(list)
    }

    fn parse_index_expression(&mut self, left: Expression) -> Option<Expression> {
        // current token is '['
        self.next_token(); // move to index expression

        let index = self.parse_expression(Precedence::Lowest)?;

        if !self.expect_peek(TokenType::Rbracket) {
            return None;
        }

        Some(Expression::IndexExpression(Box::new(IndexExpression {
            left: Box::new(left),
            index: Box::new(index),
        })))
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

