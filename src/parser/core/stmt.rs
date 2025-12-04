use crate::ast::{
    Expression, ExpressionStatement, FunctionLiteral, Identifier, IntegerLiteral, LetStatement,
    ReturnStatement, Statement, WhileStatement,
};
use crate::ast::nodes::{ForStatement, FunctionStatement, TestStatement};
use crate::debug_log;
use crate::token::TokenType;

use super::{Parser, Precedence};

impl Parser {
    pub(super) fn parse_statement(&mut self) -> Option<Statement> {
        match self.cur_token.token_type {
            TokenType::Let => {
                debug_log!("  -> parsing Let statement");
                self.parse_let_statement().map(Statement::Let)
            }
            TokenType::Return => {
                debug_log!("  -> parsing Return statement");
                self.parse_return_statement().map(Statement::Return)
            }
            TokenType::While => {
                debug_log!("  -> parsing While statement");
                self.parse_while_statement().map(Statement::While)
            }
            TokenType::For => {
                debug_log!("  -> parsing For statement");
                self.parse_for_statement().map(Statement::For)
            }
            TokenType::Function => {
                // Disambiguate between:
                //   - named function *statement*: `function foo(x) { ... }`
                //   - anonymous function *expression* used as a statement:
                //       `function(x) { ... };`
                //
                // If the next token is an identifier, we treat this as a
                // declaration; otherwise, we fall back to the regular
                // expression-statement path so the `function` token is
                // parsed via the prefix function-literal parser.
                if self.peek_token.token_type == TokenType::Ident {
                    debug_log!("  -> parsing Function statement");
                    self.parse_function_statement().map(Statement::Function)
                } else {
                    debug_log!("  -> treating `function` as expression statement");
                    let stmt = self.parse_expression_statement();
                    debug_log!("  -> parse_expression_statement (for function literal) returned: {:?}", stmt);
                    stmt.map(Statement::Expression)
                }
            }
            TokenType::Test => {
                debug_log!("  -> parsing Test statement");
                self.parse_test_statement().map(Statement::Test)
            }
            _ => {
                debug_log!("  -> default: parsing Expression statement");
                let stmt = self.parse_expression_statement();
                debug_log!("  -> parse_expression_statement returned: {:?}", stmt);
                stmt.map(Statement::Expression)
            }
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
            literal: FunctionLiteral { params, body },
        })
    }

    fn parse_test_statement(&mut self) -> Option<TestStatement> {
        // current token is 'test'
        if !self.expect_peek(TokenType::String) {
            return None;
        }

        // cur_token is now the string literal token
        let name = self.cur_token.literal.clone();

        if !self.expect_peek(TokenType::Lbrace) {
            return None;
        }

        let body = self.parse_block_statement()?;

        Some(TestStatement { name, body })
    }
}


