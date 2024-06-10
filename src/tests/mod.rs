#[cfg(test)]
use crate::parser::ast::expression::Expression::*;
use crate::types::{token::Token, Expression, Expression::*, LiteralType, TokenType};

macro_rules! new_expression {
    ($left:expr, $operator:expr,$right:expr) => {
        Expression::Binary(Box::new(Binary {
            operator: $operator,
            left: $left,
            right: $right,
        }))
    };
    ($operator:expr, $operand:expr) => {
        Expression::Unary(Box::new(Unary {
            operator: $operator,
            operand: $operand,
        }))
    };
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

#[test]
fn test_ast() {
    let mut calling_expression = new_expression!(new_literal!(LiteralType::Number(45.0)));
    let mut expression = new_expression!(
        new_expression!(
            Token::new(TokenType::Minus, "-".to_string(), None, 1),
            new_literal!(LiteralType::Number(123.0))
        ),
        Token::new(TokenType::Star, "*".to_string(), None, 1),
        new_expression!(new_literal!(LiteralType::Number(45.67)))
    );

    assert_eq!(
        "(* (- 123) (group 45.67))",
        calling_expression.print(&mut expression)
    );
}
