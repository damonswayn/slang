use crate::ast::nodes::{
    BooleanLiteral, FloatLiteral, NewExpression, ObjectLiteral, PostfixExpression, PostfixOp,
    PrefixExpression, PrefixOp, PropertyAccess,
};
use crate::ast::{
    ArrayLiteral, BlockStatement, CallExpression, Expression, ExpressionStatement, FunctionLiteral,
    Identifier, IfExpression, IndexExpression, InfixExpression, InfixOp, IntegerLiteral, Statement,
    StringLiteral,
};
use crate::debug_log;
use crate::token::TokenType;

use super::{Parser, Precedence};

impl Parser {
    // ---------- Expressions (Pratt) ----------

    pub(super) fn parse_expression(&mut self, precedence: Precedence) -> Option<Expression> {
        debug_log!(
            "parse_expression: ENTER, cur_token = {:?}, peek_token = {:?}, precedence = {:?}",
            self.cur_token,
            self.peek_token,
            precedence
        );

        let prefix = match self.prefix_fns.get(&self.cur_token.token_type).copied() {
            Some(prefix) => {
                debug_log!(
                    "parse_expression: found prefix fn for {:?}",
                    self.cur_token.token_type
                );
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
            self.cur_token,
            self.peek_token
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

    pub(super) fn parse_identifier(&mut self) -> Option<Expression> {
        Some(Expression::Identifier(Identifier {
            value: self.cur_token.literal.clone(),
        }))
    }

    pub(super) fn parse_integer_literal(&mut self) -> Option<Expression> {
        match self.cur_token.literal.parse::<i64>() {
            Ok(v) => Some(Expression::IntegerLiteral(IntegerLiteral { value: v })),
            Err(_) => {
                self.errors.push(format!(
                    "could not parse {} as integer",
                    self.cur_token.literal
                ));
                None
            }
        }
    }

    pub(super) fn parse_float_literal(&mut self) -> Option<Expression> {
        match self.cur_token.literal.parse::<f64>() {
            Ok(v) => Some(Expression::FloatLiteral(FloatLiteral { value: v })),
            Err(_) => {
                self.errors.push(format!(
                    "could not parse {} as float",
                    self.cur_token.literal
                ));
                None
            }
        }
    }

    pub(super) fn parse_grouped_expression(&mut self) -> Option<Expression> {
        // current is '('
        self.next_token(); // move to the first token inside
        let exp = self.parse_expression(Precedence::Lowest)?;
        if !self.expect_peek(TokenType::Rparen) {
            return None;
        }
        Some(exp)
    }

    pub(super) fn parse_infix_expression(&mut self, left: Expression) -> Option<Expression> {
        let operator = match self.cur_token.token_type {
            TokenType::Plus => InfixOp::Plus,
            TokenType::Minus => InfixOp::Minus,
            TokenType::Mul => InfixOp::Multiply,
            TokenType::Div => InfixOp::Divide,
            TokenType::Mod => InfixOp::Modulo,
            TokenType::LessThan => InfixOp::LessThan,
            TokenType::LessEqual => InfixOp::LessEqual,
            TokenType::GreaterThan => InfixOp::GreaterThan,
            TokenType::GreaterEqual => InfixOp::GreaterEqual,
            TokenType::Equal => InfixOp::Equals,
            TokenType::NotEqual => InfixOp::NotEquals,
            TokenType::And => InfixOp::And,
            TokenType::Or => InfixOp::Or,
            TokenType::Assign => InfixOp::Assign,
            _ => return None,
        };
        let precedence = self.cur_precedence();
        self.next_token();
        let right = self.parse_expression(precedence)?;
        Some(Expression::Infix(InfixExpression {
            left: Box::new(left),
            operator,
            right: Box::new(right),
        }))
    }

    pub(super) fn parse_prefix_expression(&mut self) -> Option<Expression> {
        let operator = match self.cur_token.token_type {
            TokenType::Bang => PrefixOp::Not,
            TokenType::Minus => PrefixOp::Negate,
            TokenType::PlusPlus => PrefixOp::PreIncrement,
            TokenType::MinusMinus => PrefixOp::PreDecrement,
            _ => return None,
        };

        self.next_token(); // move to the right-hand side

        let right = self.parse_expression(Precedence::Prefix)?;

        Some(Expression::Prefix(Box::new(PrefixExpression {
            operator,
            right: Box::new(right),
        })))
    }

    pub(super) fn parse_postfix_expression(&mut self, left: Expression) -> Option<Expression> {
        let operator = match self.cur_token.token_type {
            TokenType::PlusPlus => PostfixOp::Increment,
            TokenType::MinusMinus => PostfixOp::Decrement,
            _ => return None,
        };

        Some(Expression::Postfix(Box::new(PostfixExpression {
            left: Box::new(left),
            operator,
        })))
    }

    pub(super) fn parse_boolean_literal(&mut self) -> Option<Expression> {
        let value = matches!(self.cur_token.token_type, TokenType::True);
        Some(Expression::BooleanLiteral(BooleanLiteral { value }))
    }

    pub(super) fn parse_if_expression(&mut self) -> Option<Expression> {
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
            // Consume 'else'
            self.next_token(); // current = 'else'

            // Support `else if` by desugaring to `else { if (...) { ... } ... }`
            if self.peek_token.token_type == TokenType::If {
                // Move to 'if'
                self.next_token(); // current = 'if'

                // Parse the nested if-expression starting at this 'if'
                let nested_if_expr = self.parse_if_expression()?;

                // Wrap the nested if-expression in a block so it fits the
                // existing AST shape: `alternative: Option<BlockStatement>`.
                let stmt = Statement::Expression(ExpressionStatement {
                    expression: nested_if_expr,
                });
                let block = BlockStatement {
                    statements: vec![stmt],
                };

                Some(block)
            } else {
                // Regular `else { ... }`
                if !self.expect_peek(TokenType::Lbrace) {
                    return None;
                }
                Some(self.parse_block_statement()?)
            }
        } else {
            None
        };

        Some(Expression::If(Box::new(IfExpression {
            condition: Box::new(condition),
            consequence,
            alternative,
        })))
    }

    pub(super) fn parse_function_literal(&mut self) -> Option<Expression> {
        // current token is 'fn'
        if !self.expect_peek(TokenType::Lparen) {
            return None;
        }

        let params = self.parse_function_parameters()?;

        if !self.expect_peek(TokenType::Lbrace) {
            return None;
        }

        let body = self.parse_block_statement()?;

        Some(Expression::FunctionLiteral(FunctionLiteral {
            params,
            body,
        }))
    }

    pub(super) fn parse_call_expression(&mut self, function: Expression) -> Option<Expression> {
        debug_log!(
            "parse_call_expression: ENTER, function = {:?}, cur_token = {:?}, peek_token = {:?}",
            function,
            self.cur_token,
            self.peek_token
        );

        let arguments = self.parse_expression_list(TokenType::Rparen)?;
        debug_log!("parse_call_expression: arguments = {:?}", arguments);

        Some(Expression::CallExpression(Box::new(CallExpression {
            function: Box::new(function),
            arguments,
        })))
    }

    pub(super) fn parse_new_expression(&mut self) -> Option<Expression> {
        // current token is 'new'
        if !self.expect_peek(TokenType::Ident) {
            return None;
        }

        let class_name = Identifier {
            value: self.cur_token.literal.clone(),
        };

        if !self.expect_peek(TokenType::Lparen) {
            return None;
        }

        let arguments = self.parse_expression_list(TokenType::Rparen)?;

        Some(Expression::New(Box::new(NewExpression {
            class_name,
            arguments,
        })))
    }

    pub(super) fn parse_function_parameters(&mut self) -> Option<Vec<Identifier>> {
        let mut params = Vec::new();

        // fn() ...
        if self.peek_token.token_type == TokenType::Rparen {
            self.next_token(); // skip ')'
            return Some(params);
        }

        // first param
        self.next_token(); // current = first identifier
        params.push(Identifier {
            value: self.cur_token.literal.clone(),
        });

        // more params
        while self.peek_token.token_type == TokenType::Comma {
            self.next_token(); // skip ','
            self.next_token(); // move to next ident
            params.push(Identifier {
                value: self.cur_token.literal.clone(),
            });
        }

        if !self.expect_peek(TokenType::Rparen) {
            return None;
        }

        Some(params)
    }

    pub(super) fn parse_string_literal(&mut self) -> Option<Expression> {
        Some(Expression::StringLiteral(StringLiteral {
            value: self.cur_token.literal.clone(),
        }))
    }

    pub(super) fn parse_array_literal(&mut self) -> Option<Expression> {
        // current token is '['
        let elements = self.parse_expression_list(TokenType::Rbracket)?;
        Some(Expression::ArrayLiteral(ArrayLiteral { elements }))
    }

    pub(super) fn parse_object_literal(&mut self) -> Option<Expression> {
        // current token is '{'
        let mut properties = Vec::new();

        // Empty object: {}
        if self.peek_token.token_type == TokenType::Rbrace {
            self.next_token(); // consume '}'
            return Some(Expression::ObjectLiteral(ObjectLiteral { properties }));
        }

        loop {
            // Move to the property name identifier
            self.next_token();
            if self.cur_token.token_type != TokenType::Ident {
                self.errors.push(format!(
                    "expected identifier as object property name, got {:?}",
                    self.cur_token.token_type
                ));
                return None;
            }

            let name = Identifier {
                value: self.cur_token.literal.clone(),
            };

            if !self.expect_peek(TokenType::Colon) {
                return None;
            }

            // Move to start of value expression
            self.next_token();
            let value = self.parse_expression(Precedence::Lowest)?;
            properties.push((name, value));

            // Handle optional commas between properties, and allow a trailing comma.
            if self.peek_token.token_type == TokenType::Comma {
                // consume comma
                self.next_token();

                // Trailing comma: `{ a: 1, }`
                if self.peek_token.token_type == TokenType::Rbrace {
                    break;
                }
            } else {
                break;
            }
        }

        if !self.expect_peek(TokenType::Rbrace) {
            return None;
        }

        Some(Expression::ObjectLiteral(ObjectLiteral { properties }))
    }

    pub(super) fn parse_expression_list(&mut self, end: TokenType) -> Option<Vec<Expression>> {
        debug_log!(
            "parse_expression_list: ENTER, end = {:?}, cur_token = {:?}, peek_token = {:?}",
            end,
            self.cur_token,
            self.peek_token
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
            self.cur_token,
            self.peek_token
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

    pub(super) fn parse_index_expression(&mut self, left: Expression) -> Option<Expression> {
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

    pub(super) fn parse_property_access(&mut self, left: Expression) -> Option<Expression> {
        // current token is '.'
        if !self.expect_peek(TokenType::Ident) {
            return None;
        }

        let property = Identifier {
            value: self.cur_token.literal.clone(),
        };

        Some(Expression::PropertyAccess(Box::new(PropertyAccess {
            object: Box::new(left),
            property,
        })))
    }

    // ---------- Statements / blocks ----------

    pub(super) fn parse_block_statement(&mut self) -> Option<BlockStatement> {
        // current token is '{'
        let mut block = BlockStatement {
            statements: Vec::new(),
        };

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
