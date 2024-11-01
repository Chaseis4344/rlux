//use crate::token::Token;
use crate::types::statement::Statement;
use crate::types::token::Token;
use crate::types::token::Token;
use crate::types::Expression;
use crate::types::*;
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

macro_rules! new_assignment {
    ($name:expr, $value:expr) => {
        Expression::Assignment(Box::new(expression::Assignment {
            name: $name,
            value: $value,
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

pub struct Parser {
    tokens: Vec<Token>,
    current: i32,
}

impl Parser {
    pub fn new(tokens: Vec<Token>, current: i32) -> Parser {
        Parser { tokens, current }
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
            Err(Self::error(token, message))
        }
    }

    fn error(token: Token, message: &str) -> ParserError {
        let _ = crate::error(token.line, message.to_string());
        ParserError {
            source: token,
            cause: message.to_string(),
        }
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
