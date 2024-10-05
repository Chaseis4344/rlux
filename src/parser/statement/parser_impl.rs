use super::*;
use crate::{parser::Parser, types::statement};

impl Parser {
    /*Statement Grammar is Here Down */
    fn if_statement(&mut self) -> Result<Statement, ParserError> {
        let _ = self.consume(TokenType::LeftParen, "Expected \"(\" after if statement");
        let condition = self.expression()?;
        let _ = self.consume(TokenType::RightParen, "Expected \")\" after if statement");

        let then_branch = Box::new(self.statement()?);
        let else_branch: Option<Statement> = if self.match_token_type(vec![TokenType::Else]) {
            Some(self.statement()?)
        } else {
            None
        };

        let else_branch = Box::new(else_branch);

        Ok(Statement::If(IfStatement {
            condition,
            then_branch,
            else_branch,
        }))
    }

    fn print_statement(&mut self) -> Result<Statement, ParserError> {
        let expression = self.expression()?;
        //let expression = pass_up!(value);
        let _ = self.consume(TokenType::Semicolon, "Expect ';' after value.");

        return Ok(Statement::Print(PrintStatement { expression }));
    }

    fn while_statement(&mut self) -> Result<Statement, ParserError> {
        let _ = self.consume(TokenType::LeftParen, "Expect '(' after while.");
        let condition = self.expression()?;
        let _ = self.consume(TokenType::RightParen, "Expect ')' after while condition.");
        let body = Box::new(self.statement()?);

        Ok(Statement::While(WhileStatement { condition, body }))
    }

    // This desugars into a while loop with statements outside it
    fn for_statement(&mut self) -> Result<Statement, ParserError> {
        let _ = self.consume(TokenType::LeftParen, "Expect '(' after for.");

        let initializer: Option<Statement>;
        if self.match_token_type(vec![TokenType::Semicolon]) {
            initializer = None;
        } else if self.match_token_type(vec![TokenType::Var]) {
            initializer = Some(self.variable_decalration()?);
        } else {
            initializer = Some(self.expression_statement()?);
        }
        let _ = self.consume(TokenType::Semicolon, "Expect ';' after declaration part.");

        let mut condition: Option<Expression> = None;
        if !self.check(TokenType::Semicolon) {
            condition = Some(self.expression()?);
        }

        let _ = self.consume(TokenType::Semicolon, "Expect ';' after increment part.");

        let mut increment: Option<Expression> = None;
        if !self.check(TokenType::Semicolon) {
            increment = Some(self.expression()?);
        }

        let _ = self.consume(TokenType::RightParen, "Expect ')' after condition block.");

        let mut body = self.statement()?;

        if increment.is_some() {
            let statements: Vec<Statement> = vec![
                body,
                Statement::Expression(ExpressionStatement {
                    expression: increment.unwrap(),
                }),
            ];

            body = Statement::Block(BlockStatement { statements });
        }

        if condition.is_none() {
            condition = Some(Expression::Literal(Box::new(
                crate::types::expression::Literal {
                    value: LiteralType::Boolean(true),
                },
            )))
        }

        if condition.is_some() {
            body = Statement::While(WhileStatement {
                condition: condition.unwrap(),
                body: Box::new(body),
            });
        } else {
            crate::error(
                0,
                String::from("Interpreter Parse Error: Bad For Loop Condition Handle"),
            );
        }

        if initializer.is_some() {
            body = Statement::Block(BlockStatement {
                statements: vec![initializer.unwrap(), body],
            });
        }

        Ok(body)
    }

    ///Evaluates the expression!
    fn expression_statement(&mut self) -> Result<Statement, ParserError> {
        let expression = self.expression()?;
        return Ok(Statement::Expression(ExpressionStatement { expression }));
    }

    fn block_statement(&mut self) -> Result<Statement, ParserError> {
        let mut statements: Vec<Statement> = vec![];

        while !(self.check(TokenType::RightBrace) || self.is_at_end()) {
            statements.push(self.declaration()?);
        }

        let _ = self.consume(TokenType::RightBrace, "Expect '}' to match '{'.");

        Ok(Statement::Block(BlockStatement { statements }))
    }

    fn statement(&mut self) -> Result<Statement, ParserError> {
        if self.match_token_type(vec![TokenType::If]) {
            self.if_statement()
        } else if self.match_token_type(vec![TokenType::Print]) {
            self.print_statement()
        } else if self.match_token_type(vec![TokenType::While]) {
            self.while_statement()
        } else if self.match_token_type(vec![TokenType::For]) {
            self.for_statement()
        } else if self.match_token_type(vec![TokenType::LeftBrace]) {
            self.block_statement()
        } else {
            self.expression_statement()
        }
    }

    fn variable_decalration(&mut self) -> Result<Statement, ParserError> {
        let name = self.consume(TokenType::Identifier, "Expected Identifier for Variable")?;
        let initalizer: Expression;

        if !self.match_token_type(vec![TokenType::Equal]) {
            return Err(ParserError {
                source: self.previous(),
                cause: String::from("Expected '='"),
            });
        }

        let result = self.expression();
        if result.is_err() {
            return Err(result.unwrap_err());
        }

        initalizer = result.unwrap();
        let _ = self.consume(TokenType::Semicolon, "Expexted \";\" following statement");

        let statement = VariableStatement {
            name,
            initalizer: Some(initalizer),
        };

        Ok(Statement::Variable(statement))
    }

    pub(crate) fn declaration(&mut self) -> Result<Statement, ParserError> {
        if self.match_token_type(vec![TokenType::Var]) {
            let result = self.variable_decalration();

            if result.is_err() {
                let err = result.unwrap_err();
                eprintln!("{}", err);
                self.synchronize();
                return Err(err);
            }

            return result;
        } else {
            let result = self.statement();

            if result.is_err() {
                let err = result.unwrap_err();
                eprintln!("{}", err);
                self.synchronize();
                return Err(err);
            }

            return result;
        }
    }
}
