use super::*;
use crate::{
    macros::error_check,
    parser::Parser,
    types::token::Token,
};

impl Parser {
    /*Statement Grammar is Here Down */
    fn if_statement(&mut self) -> Result<Statement, ParserError> {
        let consumed = self.consume(TokenType::LeftParen, "Expected \"(\" after if statement");
        error_check!(consumed);

        let condition = self.expression()?;

        let consumed = self.consume(TokenType::RightParen, "Expected \")\" after if statement");
        error_check!(consumed);

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
    fn return_statement(&mut self) -> Result<Statement, ParserError> {
        let keyword: Token = self.previous();
        let value: Option<Expression> = if self.match_token_type(vec![TokenType::Semicolon]) {
            None
        } else {
            Some(self.expression()?)
        };

        let consumed = self.consume(TokenType::Semicolon, "Expected ';' after return");
        error_check!(consumed);

        Ok(Statement::Return(ReturnStatement { keyword, value }))
    }
    /*
        fn print_statement(&mut self) -> Result<Statement, ParserError> {
            let expression = self.expression()?;
            //let expression = pass_up!(value);
            let _ = self.consume(TokenType::Semicolon, "Expect ';' after value.");

            Ok(Statement::Print(PrintStatement { expression }))
        }
    */
    fn while_statement(&mut self) -> Result<Statement, ParserError> {
        let _ = self.consume(TokenType::LeftParen, "Expect '(' after while.")?;
        let condition = self.expression()?;
        let _ = self.consume(TokenType::RightParen, "Expect ')' after while condition.")?;
        let body = Box::new(self.statement()?);

        Ok(Statement::While(WhileStatement { condition, body }))
    }

    // This desugars into a while loop with statements outside it
    fn for_statement(&mut self) -> Result<Statement, ParserError> {
        let _ = self.consume(TokenType::LeftParen, "Expect '(' after for.");

        let initializer: Option<Statement> = if self.match_token_type(vec![TokenType::Semicolon]) {
            None
        } else if self.match_token_type(vec![TokenType::Var]) {
            Some(self.variable_decalration()?)
        } else {
            Some(self.expression_statement()?)
        };

        let condition: Option<Expression> = if self.check(TokenType::Semicolon) {
            None
        } else {
            Some(self.expression()?)
        };

        let _ = self.consume(TokenType::Semicolon, "Expect ';' after increment part.");

        let mut increment: Option<Expression> = None;
        if !self.check(TokenType::RightParen) {
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

        //Is there a condition Present? if not just eval to true every time
        let condition = if condition.is_none() {
            Some(Expression::Literal(Box::new(
                crate::types::expression::Literal {
                    value: LiteralType::Boolean(true),
                },
            )))
        } else {
            condition
        };

        //No Error path needed since all errors are pre-empted
        body = Statement::While(WhileStatement {
            condition: condition.expect("Condition has been set to None"),
            body: Box::new(body),
        });

        if initializer.is_some() {
            body = Statement::Block(BlockStatement {
                statements: vec![initializer.unwrap(), body],
            });
        }

        Ok(body)
    }

    ///Evaluates the expression in the Syntax!
    fn expression_statement(&mut self) -> Result<Statement, ParserError> {
        let expression = self.expression()?;
        Ok(Statement::Expression(ExpressionStatement { expression }))
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
            let returned = self.if_statement();
            error_check!(returned);
            returned
        }
        /* else if self.match_token_type(vec![TokenType::Print]) {
            self.print_statement()
        } */
        else if self.match_token_type(vec![TokenType::Return]) {
            let returned = self.return_statement();
            error_check!(returned);
            returned
        } else if self.match_token_type(vec![TokenType::While]) {
            let returned = self.while_statement();
            error_check!(returned);
            returned
        } else if self.match_token_type(vec![TokenType::For]) {
            let returned = self.for_statement();
            error_check!(returned);
            returned
        } else if self.match_token_type(vec![TokenType::LeftBrace]) {
            let returned = self.block_statement();
            error_check!(returned);
            returned
        } else {
            let returned = self.expression_statement();
            error_check!(returned);
            returned
        }
    }

    fn variable_decalration(&mut self) -> Result<Statement, ParserError> {
        let name = self.consume(TokenType::Identifier, "Expected Identifier for Variable");
        error_check!(name);

        //Error was handeled at runtime, we can now expect the name to be present
        let name = name.expect("Variable Identifier is erroring");

        let initalizer: Option<Expression> = if self.match_token_type(vec![TokenType::Equal]) {
            Some(self.expression()?)
        } else {
            None
        };

        let consumed = self.consume(TokenType::Semicolon, "Expected \";\" following statement");
        // println!("{:?}", consumed);
        // This allows for loops to have assignment statements inside their heads without the
        // parser throwing a fit at the user
        if let Err(error) = consumed 
            && error.source.token_type != TokenType::For
                && error.source.lexeme.to_lowercase() != "print"
            {
                Parser::error(error.source, &error.cause);
            }
        

        let statement = VariableStatement { name, initalizer };

        Ok(Statement::Variable(statement))
    }

    fn function_declaration(&mut self, kind: String) -> Result<Statement, ParserError> {
        let name = self.consume(TokenType::Identifier, &format!("Expect {kind} name"))?;
        let mut parameters: Vec<Token> = vec![];

        let _ = self.consume(TokenType::LeftParen, &format!("Expect ( after {kind}"));
        if !self.match_token_type(vec![TokenType::RightParen]) {
            while {
                if parameters.len() + 1 > u64::MAX.try_into().unwrap() {
                    Parser::error(
                        self.peek(),
                        &format!(
                            "Cannot have more than {} parameters in a function",
                            u64::MAX
                        ),
                    );
                    return Err(ParserError {
                        source: self.peek(),
                        cause: format!(
                            "Cannot have more than {} parameters in a function",
                            u64::MAX
                        ),
                    });
                }
                parameters.push(self.consume(TokenType::Identifier, "Expected Parameter name")?);
                self.match_token_type(vec![TokenType::Comma])
            } {}
            let _ = self.consume(
                TokenType::RightParen,
                &format!("Expected ) after parameters for {kind}"),
            )?;
        }
        let consumed = self.consume(
            TokenType::LeftBrace,
            "Expected \'{\' after function statement",
        );
        error_check!(consumed);
        let body = self.block_statement()?;
        let body: Vec<Statement> = match body {
            Statement::Block(block) => block.statements,
            _ => {
                println!("Bad Path! Block is nothing! ");
                vec![]
            }
        };
        Ok(Statement::Function(FunctionStatement {
            name,
            body,
            parameters,
        }))
    }

    pub(crate) fn declaration(&mut self) -> Result<Statement, ParserError> {
        if self.match_token_type(vec![TokenType::Fun]) {
            self.function_declaration(String::from("function"))
        } else if self.match_token_type(vec![TokenType::Var]) {
            let result = self.variable_decalration();

            if let Err(err) = result {
                println!("{}", err);
                self.synchronize();
                return Err(err);
            }

            result
        } else {
            let result = self.statement();

            if let Err(err) = result {
                println!("{}", err);
                self.synchronize();
                return Err(err);
            }

            result
        }
    }
}
