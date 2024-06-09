//use crate::token::Token;
use crate::types::token::Token;
use crate::types::Expression;
use crate::types::*;
pub(crate) mod ast;
mod interpreter;

macro_rules! new_ternary {
    ($eval:expr, $lhs:expr,  $rhs:expr) => {
        Expression::Ternary(Box::new(ast::Ternary {
            evaluator: $eval,
            left: $lhs,
            right: $rhs,
        }))
    };
}

macro_rules! new_expression {
    ($left:expr, $operator:expr,$right:expr) => {
        Expression::Binary(Box::new(ast::Binary {
            operator: $operator,
            left: $left,
            right: $right,
        }))
    };
    ($operator:expr, $operand:expr) => {
        Expression::Unary(Box::new(ast::Unary {
            operator: $operator,
            operand: $operand,
        }))
    };
    ($expression:expr) => {
        Expression::Grouping(Box::new(ast::Grouping {
            expression: $expression,
        }))
    };
}

macro_rules! new_literal {
    ($value:expr) => {
        Expression::Literal(Box::new(ast::Literal { value: $value }))
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
            if !self.match_token_type(vec![TokenType::Colon]) {
                /*return Parse Error for user */
                return Err(ParserError {
                    source: self.peek(),
                });
            }

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

    fn advance(&mut self) -> Option<Token> {
        if !self.is_at_end() {
            self.current += 1;
            None
        } else {
            Some(self.previous())
        }
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

    fn consume(&mut self, type_: TokenType, message: &str) -> Result<Token, (Token, String)> {
        if self.check(type_) {
            Ok(self.advance().unwrap())
        } else {
            Err((self.peek(), message.to_string()))
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

    pub(crate) fn parse(&mut self) -> Option<Expression> {
        match self.expression() {
            Ok(exp) => Some(exp),
            Err(err) => {
                eprintln!("ParserError: \n\t{}", err);
                None
            }
        }
    }
}
