use crate::ast::Expression;
use crate::token::Token;
use crate::types::*;
macro_rules! new_expression {
    ($left:expr, $operator:expr,$right:expr) => {
        crate::ast::Expression::Binary(Box::new(crate::ast::Binary {
            operator: $operator,
            left: $left,
            right: $right,
        }))
    };
    ($operator:expr, $operand:expr) => {
        crate::ast::Expression::Unary(Box::new(crate::ast::Unary {
            operator: $operator,
            operand: $operand,
        }))
    };
    ($expression:expr) => {
        crate::ast::Expression::Grouping(Box::new(crate::ast::Grouping {
            expression: $expression,
        }))
    };
}

macro_rules! new_literal {
    ($value:expr) => {
        crate::ast::Expression::Literal(Box::new(crate::ast::Literal { value: $value }))
    };
}

pub struct Parser {
    tokens: Vec<Token>,
    current: i32,
}

impl Parser {
    fn expression(&mut self) -> Expression {
        self.equality()
    }

    fn equality(&mut self) -> Expression {
        let mut expression: Expression = self.comparison();
        while self.match_token_type(vec![TokenType::BangEqual, TokenType::EqualEqual]) {
            let operator = self.previous();
            let right = self.comparison();
            expression = new_expression!(expression, operator, right);
        }

        expression
    }

    fn comparison(&mut self) -> Expression {
        let mut expression = self.term();
        while self.match_token_type(vec![
            TokenType::Greater,
            TokenType::GreaterEqual,
            TokenType::Less,
            TokenType::LessEqual,
        ]) {
            let operator = self.previous();
            let right = self.term();
            expression = new_expression!(expression, operator, right);
        }

        expression
    }
    fn term(&mut self) -> Expression {
        let mut expression = self.factor();

        while self.match_token_type(vec![TokenType::Minus, TokenType::Plus]) {
            let operator = self.previous();
            let right = self.factor();
            expression = new_expression!(expression, operator, right);
        }

        expression
    }

    fn unary(&mut self) -> Expression {
        if self.match_token_type(vec![TokenType::Bang, TokenType::Minus]) {
            let operator = self.previous();
            let right = self.unary();
            return new_expression!(operator, right);
        }

        self.primary()
    }

    fn factor(&mut self) -> Expression {
        let mut expression = self.unary();

        while self.match_token_type(vec![TokenType::Slash, TokenType::Star]) {
            let operator = self.previous();
            let right = self.unary();
            expression = new_expression!(expression, operator, right);
        }

        expression
    }

    fn primary(&mut self) -> Expression {
        if self.match_token_type(vec![
            TokenType::False,
            TokenType::True,
            TokenType::Nil,
            TokenType::Number,
            TokenType::String,
        ]) {
            let underlying_value = self
                .previous()
                .literal
                .expect("Expected Literal, received None");
            match underlying_value {
                LiteralType::Number(num) => return new_literal!(LiteralType::Number(num)),
                LiteralType::String(string) => return new_literal!(LiteralType::String(string)),
                LiteralType::Boolean(boolean) => {
                    return new_literal!(LiteralType::Boolean(boolean))
                }
                LiteralType::Nil => return new_literal!(LiteralType::Nil),
            }
        }

        new_literal!(LiteralType::Nil)
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

    fn error(token: Token, message: &str) -> Result<i32, &str> {
        crate::error(token.line, message.to_string());
        Err("Err")
    }
}
