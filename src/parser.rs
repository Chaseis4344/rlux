macro_rules! new_expression {
    ($left:expr, $operator:expr,$right:expr) => {
        ast::Expression::Binary(Box::new(ast::Binary {
            operator: $operator,
            left: $left,
            right: $right,
        }))
    };
    ($operator:expr, $operand:expr) => {
        ast::Expression::Unary(Box::new(ast::Unary {
            operator: $operator,
            operand: $operand,
        }))
    };
    ($expression:expr) => {
        ast::Expression::Grouping(Box::new(ast::Grouping {
            expression: $expression,
        }))
    };
}

macro_rules! new_literal {
    ($value:expr) => {
        ast::Expression::Literal(Box::new(ast::Literal { value: $value }))
    };
}

pub struct Parser {
    tokens: Vec<Token>,
    current: i32,
}

impl Parser {
    fn expression() -> Expression {
        equality()
    }

    fn equality() -> Expression {
        let mut expr: Expression = comparison();
        while (match_token_type(vec![TokenType::BangEqual, TokenType::EqualEqual])) {
            let operator = previous();
            let right = comparison();
            expr = new_expression!(expr, operator, right);
        }

        expr
    }

    fn match_token_type(types: Vec<TokenType>) -> bool {
        for token_type in types {
            if check(token_type) {
                advance();
                return true;
            }
        }

        return false;
    }

    fn check(token_type: TokenType) {
        if is_at_end() {
            false
        } else {
            peek().token_type == token_type
        }
    }

    fn advance(&mut self) -> Option<Token> {
        if !is_at_end() {
            sef.current += 1;
            None
        } else {
            Some(previous())
        }
    }

    fn is_at_end() -> bool {
        peek().token_type == TokenType::EOF
    }

    fn peek(&self) -> Token {
        self.tokens[self.current]
    }

    fn previous(&self) -> Token {
        self.tokens[self.current - 1]
    }
}
