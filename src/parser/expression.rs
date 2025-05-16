use crate::parser::Parser;
use crate::types::{expression::*, token::Token, Expression, LiteralType, ParserError, TokenType};

//These macros create new types of expressions, this is so the code is understandable

macro_rules! new_ternary {
    ($eval:expr, $lhs:expr,  $rhs:expr) => {
        Expression::Ternary(Box::new(Ternary {
            evaluator: $eval,
            left: $lhs,
            right: $rhs,
        }))
    };
}

macro_rules! new_call {
    ($callee:expr, $paren:expr, $arguments: expr) => {
        Expression::Call(Box::new(Call {
            callee: $callee,
            paren: $paren,
            arguments: $arguments,
        }))
    };
}

macro_rules! new_logical {
    ($op:expr, $lhs:expr,  $rhs:expr) => {
        Expression::Logical(Box::new(Logical {
            left: $lhs,
            right: $rhs,
            operator: $op,
        }))
    };
}

macro_rules! new_assignment {
    ($name:expr, $value:expr) => {
        Expression::Assignment(Box::new(Assignment {
            name: $name,
            value: $value,
        }))
    };
}

macro_rules! new_binary {
    ($left:expr, $operator:expr,$right:expr) => {
        Expression::Binary(Box::new(Binary {
            operator: $operator,
            left: $left,
            right: $right,
        }))
    };
}

macro_rules! new_unary {
    ($operator:expr, $operand:expr) => {
        Expression::Unary(Box::new(Unary {
            operator: $operator,
            operand: $operand,
        }))
    };
}

macro_rules! new_grouping {
    ($expression:expr) => {
        Expression::Grouping(Box::new(Grouping {
            expression: $expression,
        }))
    };
}

macro_rules! new_literal {
    ($value:expr) => {
        Expression::Literal(Box::new(Literal { value: $value }))
    };
}

impl Parser {
    pub(crate) fn expression(&mut self) -> Result<Expression, ParserError> {
        self.or()
    }

    fn or(&mut self) -> Result<Expression, ParserError> {
        let mut expression = self.and()?;

        while self.match_token_type(vec![TokenType::Or]) {
            let operator = self.previous();
            let right = self.and()?;
            expression = new_logical!(operator, expression, right);
        }

        Ok(expression)
    }

    fn and(&mut self) -> Result<Expression, ParserError> {
        let mut expression = self.ternary()?;

        while self.match_token_type(vec![TokenType::And]) {
            let operator = self.previous();
            let right = self.and()?;
            expression = new_logical!(operator, expression, right);
        }

        Ok(expression)
    }

    fn ternary(&mut self) -> Result<Expression, ParserError> {
        let mut ternary = self.assignment()?;

        while self.match_token_type(vec![TokenType::Question]) {
            let lhs = self.assignment()?;

            /*Consume ":"/ Enforces Grammar */
            let _ = self.consume(
                TokenType::Colon,
                &(format!("Expected \":\" instead of {}", self.peek())),
            );

            let rhs = self.assignment()?;
            ternary = new_ternary!(ternary, lhs, rhs);
        }

        Ok(ternary)
    }

    fn assignment(&mut self) -> Result<Expression, ParserError> {
        let expression = self.equality()?;

        if self.match_token_type(vec![TokenType::Equal]) {
            let equals = self.previous();
            let value: Expression = self.assignment()?;

            let _ = self.consume(TokenType::Semicolon, "Expected ';' after assignement");

            match expression.clone() {
                Expression::Variable(var) => {
                    let name = var.name;
                    return Ok(Expression::Assignment(Box::new(Assignment { name, value })));
                }
                _ => {
                    return Err(ParserError {
                        source: equals,
                        cause: String::from("Invalid Assignment Target"),
                    });
                }
            }
        }

        Ok(expression)
    }

    fn equality(&mut self) -> Result<Expression, ParserError> {
        let mut expression = self.comparison()?;

        while self.match_token_type(vec![TokenType::BangEqual, TokenType::EqualEqual]) {
            let operator = self.previous();
            let right = self.comparison()?;
            expression = new_binary!(expression, operator, right);
        }

        Ok(expression)
    }

    fn comparison(&mut self) -> Result<Expression, ParserError> {
        let mut expression = self.term()?;

        while self.match_token_type(vec![
            TokenType::Greater,
            TokenType::GreaterEqual,
            TokenType::Less,
            TokenType::LessEqual,
        ]) {
            let operator = self.previous();
            let right = self.term()?;
            expression = new_binary!(expression, operator, right);
        }

        Ok(expression)
    }

    fn term(&mut self) -> Result<Expression, ParserError> {
        let mut expression = self.factor()?;

        while self.match_token_type(vec![TokenType::Minus, TokenType::Plus]) {
            let operator = self.previous();
            let right = self.factor()?;
            expression = new_binary!(expression, operator, right);
        }

        Ok(expression)
    }

    fn factor(&mut self) -> Result<Expression, ParserError> {
        let mut expression = self.unary()?;

        while self.match_token_type(vec![TokenType::Slash, TokenType::Star]) {
            let operator = self.previous();
            let right = self.unary()?;
            expression = new_binary!(expression, operator, right);
        }

        Ok(expression)
    }

    fn unary(&mut self) -> Result<Expression, ParserError> {
        if self.match_token_type(vec![TokenType::Bang, TokenType::Minus]) {
            let operator = self.previous();
            let right = self.unary()?;
            let expression = new_unary!(operator, right);
            println!("{}", expression.clone());
            return Ok(expression);
        }

        self.call()
    }

    fn call(&mut self) -> Result<Expression, ParserError> {
        let mut expression = self.primary();

        loop {
            if self.match_token_type(vec![TokenType::LeftParen]) {
                expression =
                    self.finish_call(expression.expect("Expression Expected, ParserError Found"));
            } else {
                break;
            }
        }

        expression
    }

    fn finish_call(&mut self, callee: Expression) -> Result<Expression, ParserError> {
        let mut arguments: Vec<Expression> = vec![];
        if !self.check(TokenType::RightParen) {
            //Secretly a do while
            while {
                //Body
                arguments.push(self.expression()?);

                //Then Eval Condition
                self.match_token_type(vec![TokenType::Comma])
            } {}
        }

        let paren: Token = self.consume(TokenType::RightParen, "Expect ')' after arguments ")?;
        //TODO: Relocate this somewhere that makes more smeantic sense
        let _ = self.consume(
            TokenType::Semicolon,
            "Expect Semicolona after print statement",
        );
        Ok(new_call!(callee, paren, arguments))
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
                LiteralType::Callable(_) => {
                    return Err(ParserError {
                        source: self.previous(),
                        cause: String::from("Cannot Evaluate a function from primary!"),
                    })
                }
            };

            Ok(return_val)
        } else if self.match_token_type(vec![TokenType::LeftParen]) {
            let expression = self.expression()?;
            let _ = self.consume(TokenType::RightParen, "Expect \')\' after expression.");

            return Ok(new_grouping!(expression));
        } else if self.match_token_type(vec![TokenType::Identifier]) {
            return Ok(Expression::Variable(Box::new(Variable {
                name: self.previous(),
            })));
        } else {
            return Err(ParserError {
                source: self.peek(),
                cause: String::from("Unexpected Character"),
            });
        }
    }
}
