use std::result;

//use crate::token::Token;
use crate::types::token::Token;
use crate::types::Expression;
use crate::types::*;
use expression::Variable;
use statement::{ExpressionStatement, PrintStatement, Statement, VariableStatement};

pub(crate) mod expression;
pub(crate) mod interpreter;
pub(crate) mod statement;

macro_rules! new_ternary {
    ($eval:expr, $lhs:expr,  $rhs:expr) => {
        Expression::Ternary(Box::new(expression::Ternary {
            evaluator: $eval,
            left: $lhs,
            right: $rhs,
        }))
    };
}

macro_rules! new_expression {
    ($left:expr, $operator:expr,$right:expr) => {
        Expression::Binary(Box::new(expression::Binary {
            operator: $operator,
            left: $left,
            right: $right,
        }))
    };
    ($operator:expr, $operand:expr) => {
        Expression::Unary(Box::new(expression::Unary {
            operator: $operator,
            operand: $operand,
        }))
    };
    ($expression:expr) => {
        Expression::Grouping(Box::new(expression::Grouping {
            expression: $expression,
        }))
    };
}

macro_rules! new_literal {
    ($value:expr) => {
        Expression::Literal(Box::new(expression::Literal { value: $value }))
    };
}

macro_rules! pass_up {
    ($right: ident) => {
        match $right {
            Ok(good) => good,
            Err(err) => {
                return Err(err);
            }
        }
    };
}

pub struct Parser {
    tokens: Vec<Token>,
    current: i32,
}

impl Parser {
    pub fn new(tokens: Vec<Token>, current: i32) -> Parser {
        Parser { tokens, current }
    }

    /*Statement Grammar is Here Down */
    fn print_statement(&mut self) -> Result<Statement, ParserError> {
        let value = self.expression();
        let expression = pass_up!(value);
        let _ = self.consume(TokenType::Semicolon, "Expect ';' after value.");

        return Ok(Statement::Print(PrintStatement { expression }));
    }

    fn expression_statement(&mut self) -> Result<Statement, ParserError> {
        let value = self.expression();
        let expression = pass_up!(value);
        let _ = self.consume(TokenType::Semicolon, "Expect ';' after value.");

        return Ok(Statement::Expression(ExpressionStatement { expression }));
    }

    fn statement(&mut self) -> Result<Statement, ParserError> {
        if self.match_token_type(vec![TokenType::Print]) {
            self.print_statement()
        } else {
            self.expression_statement()
        }
    }

    fn variable_decalration(&mut self) -> Result<Statement, ParserError> {
        let name = self.consume(TokenType::Identifier, "Expected Identifier for Variable");
        let name = pass_up!(name);

        let initalizer: Expression;
        if !self.match_token_type(vec![TokenType::Equal]) {
            return Err(ParserError {
                source: self.peek(),
            });
        }

        let result = self.expression();
        if result.is_err() {
            return Err(result.unwrap_err());
        }

        initalizer = result.unwrap();
        self.consume(TokenType::Semicolon, "Expexted \";\" following statement");

        let statement = VariableStatement {
            name,
            initalizer: Some(initalizer),
        };

        Ok(Statement::Variable(statement))
    }

    fn declaration(&mut self) -> Result<Statement, ParserError> {
        if self.match_token_type(vec![TokenType::Var]) {
            let result = self.variable_decalration();

            if result.is_err() {
                let err = result.unwrap_err();
                eprintln!("{}", err);
                self.synchronize();
                return self.declaration();
            }

            return result;
        } else {
            let result = self.statement();

            if result.is_err() {
                let err = result.unwrap_err();
                eprintln!("{}", err);
                self.synchronize();
                return self.declaration();
            }

            return result;
        }
    }

    /*Expression Grammar is Here Down */
    fn expression(&mut self) -> Result<Expression, ParserError> {
        self.ternary()
    }

    fn ternary(&mut self) -> Result<Expression, ParserError> {
        let ternary = self.equality();
        let mut ternary = pass_up!(ternary);

        while self.match_token_type(vec![TokenType::Question]) {
            let lhs = self.equality();
            let lhs = pass_up!(lhs);

            /*Consume ":"/ Enforces Grammar */
            self.consume(
                TokenType::Colon,
                &(format!("Expected \":\" instead of {}", self.peek())),
            );

            let rhs = self.equality();
            let rhs = pass_up!(rhs);
            ternary = new_ternary!(ternary, lhs, rhs);
        }

        Ok(ternary)
    }

    fn equality(&mut self) -> Result<Expression, ParserError> {
        let expression = self.comparison();
        let mut expression = pass_up!(expression);

        while self.match_token_type(vec![TokenType::BangEqual, TokenType::EqualEqual]) {
            let operator = self.previous();
            let right = self.comparison();
            let right = pass_up!(right);
            expression = new_expression!(expression, operator, right);
        }

        Ok(expression)
    }

    fn comparison(&mut self) -> Result<Expression, ParserError> {
        let expression = self.term();
        let mut expression = pass_up!(expression);
        while self.match_token_type(vec![
            TokenType::Greater,
            TokenType::GreaterEqual,
            TokenType::Less,
            TokenType::LessEqual,
        ]) {
            let operator = self.previous();
            let right = self.term();
            let right = pass_up!(right);
            expression = new_expression!(expression, operator, right);
        }

        Ok(expression)
    }
    fn term(&mut self) -> Result<Expression, ParserError> {
        let expression = self.factor();
        let mut expression = pass_up!(expression);

        while self.match_token_type(vec![TokenType::Minus, TokenType::Plus]) {
            let operator = self.previous();
            let right = self.factor();
            let right = pass_up!(right);
            expression = new_expression!(expression, operator, right);
        }

        Ok(expression)
    }

    fn unary(&mut self) -> Result<Expression, ParserError> {
        if self.match_token_type(vec![TokenType::Bang, TokenType::Minus]) {
            let operator = self.previous();
            let right = self.unary();
            let right = pass_up!(right);
            return Ok(new_expression!(operator, right));
        }

        match self.primary() {
            Ok(expression) => Ok(expression),
            Err(_) => Err(Self::error(self.peek(), "Eval error:")),
        }
    }

    fn factor(&mut self) -> Result<Expression, ParserError> {
        let expression = self.unary();
        let mut expression = pass_up!(expression);

        while self.match_token_type(vec![TokenType::Slash, TokenType::Star]) {
            let operator = self.previous();
            let right = self.unary();
            let right = pass_up!(right);
            expression = new_expression!(expression, operator, right);
        }

        Ok(expression)
    }

    fn primary(&mut self) -> Result<Expression, ParserError> {
        if self.match_token_type(vec![
            TokenType::False,
            TokenType::True,
            TokenType::Nil,
            TokenType::Number,
            TokenType::String,
        ]) {
            /*
            This will always work bc I am garunteeing that literal types
            will always have a literal value attatched
            */
            let underlying_value = self.previous().literal.unwrap();

            let return_val = match underlying_value {
                LiteralType::Number(num) => new_literal!(LiteralType::Number(num)),
                LiteralType::String(string) => new_literal!(LiteralType::String(string)),
                LiteralType::Boolean(boolean) => {
                    new_literal!(LiteralType::Boolean(boolean))
                }
                LiteralType::Nil => new_literal!(LiteralType::Nil),
            };
            return Ok(return_val);
        } else if self.match_token_type(vec![TokenType::LeftParen]) {
            let expression = self.expression();
            let expression = pass_up!(expression);
            let _ = self.consume(TokenType::LeftParen, "Expect ')' after expression.");
            return Ok(new_expression!(expression));
        } else if self.match_token_type(vec![TokenType::Identifier]) {
            return Ok(new_expression!(Expression::Variable(Box::new(Variable {
                name: self.previous()
            }))));
        } else {
            return Err(ParserError {
                source: self.peek(),
            });
        }
    }

    fn match_token_type(&mut self, types: Vec<TokenType>) -> bool {
        for token_type in types {
            if self.check(token_type) {
                self.advance();
                return true;
            }
        }

        return false;
    }

    fn check(&self, token_type: TokenType) -> bool {
        if self.is_at_end() {
            false
        } else {
            self.peek().token_type == token_type
        }
    }

    fn advance(&mut self) -> Token {
        if !self.is_at_end() {
            self.current += 1;
        }
        self.previous()
    }

    fn is_at_end(&self) -> bool {
        self.peek().token_type == TokenType::Eof
    }

    fn peek(&self) -> Token {
        self.tokens[self.current as usize].clone()
    }

    fn previous(&self) -> Token {
        self.tokens[(self.current - 1) as usize].clone()
    }

    fn consume(&mut self, type_: TokenType, message: &str) -> Result<Token, ParserError> {
        if self.check(type_) {
            Ok(self.advance())
        } else {
            let mut token = self.peek();
            token.line -= 1;
            Err(Self::error(token, message))
        }
    }

    fn error(token: Token, message: &str) -> ParserError {
        let _ = crate::error(token.line, message.to_string());
        ParserError { source: token }
    }

    fn synchronize(&mut self) {
        self.advance();
        while !self.is_at_end() {
            if self.previous().token_type == TokenType::Semicolon {
                return;
            };

            match self.peek().token_type {
                TokenType::Class
                | TokenType::Fun
                | TokenType::Var
                | TokenType::For
                | TokenType::If
                | TokenType::While
                | TokenType::Print
                | TokenType::Return => {
                    return;
                }

                _ => {}
            }

            self.advance();
        }
    }

    pub(crate) fn parse(&mut self) -> Vec<Statement> {
        let mut statements: Vec<Statement> = vec![];
        while !self.is_at_end() {
            let state = self.declaration();
            match state {
                Ok(statement) => {
                    statements.push(statement);
                }
                Err(_err) => {}
            }
        }

        statements
    }
}
