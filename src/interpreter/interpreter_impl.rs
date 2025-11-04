use crate::{
    enviroment::Enviroment,
    interpreter::{
        Interpreter,
        InterpreterVisitor,
    },
    types::{
        Expression,
        LiteralType,
        TokenType,
        expression::{
            Call,
            *,
        },
        lux_functions::{
            Callable as CallableTrait,
            Functions,
            Functions::Clock as OuterClock,
            clock::Clock,
            print::Print,
        },
        statement::{
            ReturnStatement,
            Statement,
        },
    },
};
use std::collections::HashMap;
// fun -> LiteralType | fun

impl Interpreter {
    pub(crate) fn evaluate(&mut self, expr: &mut Expression) -> LiteralType {
        use crate::interpreter::interpreter_traits::Visitable;
        expr.accept(self)
    }

    pub(crate) fn new() -> Interpreter {
        let map = HashMap::new();
        let mut globals = Enviroment {
            enclosing: None,
            variable_map: map,
        };
        //Inject built-ins (native functions) into enviroment
        let clock = OuterClock(Clock {});
        let print = crate::types::lux_functions::Functions::Print(Print {});

        globals.define(String::from("clock"), LiteralType::Callable(clock));
        globals.define(String::from("print"), LiteralType::Callable(print));

        let enviroment = Box::new(globals.clone());
        Interpreter {
            enviroment,
            globals,
        }
    }

    ///Hand over between the Parser and the Interpreter
    pub(crate) fn execute(&mut self, mut statement: Statement) -> Option<Expression> {
        use crate::parser::statement::Visitable as ParserVisitable;
        //If we come back up and it's a return statement then return the expression
        if let Statement::Return(ReturnStatement {
            keyword: _token,
            value: expr,
        }) = statement.accept(self)
        {
            // println!("Returned from second: {:?}", expr);
            return expr;
        }
        None
    }

    pub(crate) fn execute_block_in_env(
        statements: Vec<Statement>,
        enviroment: Enviroment,
    ) -> Option<Expression> {
        //Wrap
        let mut temporary_int: Interpreter = Interpreter::new();
        temporary_int.enviroment = Box::new(enviroment);

        //Execute
        for statement in statements {
            temporary_int.execute(statement);
        }

        None
    }
    pub(crate) fn execute_block(&mut self, statements: Vec<Statement>) -> Option<ReturnStatement> {
        //Wrap
        self.enviroment = Box::new(Enviroment {
            enclosing: Some(self.enviroment.to_owned()),
            variable_map: HashMap::new(),
        });

        //Execute
        for statement in statements {
            self.execute(statement);
        }
        //Unwrap
        self.enviroment = self.enviroment.enclosing.to_owned().unwrap();
        None
    }
}

///Logic for how the Interpreter acts with each operator or Token
impl InterpreterVisitor<LiteralType> for Interpreter {
    fn visit_binary(&mut self, bin: &mut Binary) -> LiteralType {
        let left = self.evaluate(&mut bin.left);
        let right = self.evaluate(&mut bin.right);
        let operator = &bin.operator;

        //We can abstract all this logic away to rust's traits
        match operator.token_type {
            TokenType::Plus => left + right,
            TokenType::Star => left * right,
            TokenType::Slash => left / right,
            TokenType::Minus => left - right,
            TokenType::Greater => LiteralType::Boolean(left > right),
            TokenType::GreaterEqual => LiteralType::Boolean(left >= right),
            TokenType::Less => LiteralType::Boolean(left < right),
            TokenType::LessEqual => LiteralType::Boolean(left <= right),
            TokenType::EqualEqual => LiteralType::Boolean(left == right),
            TokenType::BangEqual => LiteralType::Boolean(left != right),
            _ => {
                crate::error(operator.line, "Operator not defined for this operation".to_string());
                LiteralType::Nil
            },
        }
    }
    fn visit_grouping(&mut self, group: &mut Grouping) -> LiteralType {
        self.evaluate(&mut group.expression)
    }
    fn visit_literal(&mut self, lit: &mut Literal) -> LiteralType {
        lit.value.to_owned()
    }
    fn visit_ternary(&mut self, tern: &mut Ternary) -> LiteralType {
        let evaluator = self.evaluate(&mut tern.evaluator);
        let left = &mut tern.left;
        let right = &mut tern.right;

        match evaluator {
            LiteralType::Boolean(truthy) => {
                if truthy {
                    self.evaluate(left)
                } else {
                    self.evaluate(right)
                }
            }
            _ => evaluator,
        }
    }
    fn visit_unary(&mut self, unary: &mut Unary) -> LiteralType {
        let right = self.evaluate(&mut unary.operand);

        match unary.operator.token_type {
            TokenType::Minus => match right {
                LiteralType::Number(num) => LiteralType::Number(-num),
                _ => right,
            },
            TokenType::Bang => match right {
                LiteralType::Boolean(boolean) => LiteralType::Boolean(!boolean),
                _ => right,
            },
           _ => {
                crate::error(unary.operator.line, "Operator not defined for this operation".to_string());
                LiteralType::Nil
            },
        }
    }

    fn visit_variable(&mut self, var: &mut Variable) -> LiteralType {
        //!Returns the value of a variable, will return NIL if nothing is found
        let var = var.to_owned();
        let name = &var.name.lexeme;
        let result: Result<LiteralType, std::env::VarError> =
            self.enviroment.to_owned().get(name.to_string());
        // println!("{:?}", self.enviroment);
        if let Ok(item) = result {
            item
        } else {
            //TODO: This branch keeps getting called twice, need to investigate why
            //Nothing was found so we return nothing
            // println!("{:?}",self.enviroment.clone());
            crate::error(
                var.name.line,
                "Variable not found: ".to_owned() + &var.name.lexeme,
            );
            LiteralType::Nil
        }
    }

    fn visit_assignment(&mut self, assign: &mut Assignment) -> LiteralType {
        //Decompose assignment to avoid excess cloning
        let (name, value) = (assign.name.to_owned(), &mut assign.value);

        //Evaluate expression inside
        let value = self.evaluate(value);

        //Copy the value then echo out for the rest of the syntax tress
        self.enviroment
            .assign(name.lexeme, value.clone(), name.line);

        value
    }

    fn visit_logical(&mut self, logical: &mut Logical) -> LiteralType {
        let left: LiteralType = self.evaluate(&mut logical.left);

        let left_bool = match left {
            LiteralType::Boolean(val) => val,
            _ => panic!("Cannot coerce Non-Boolean to Boolean"),
        };

        //Short Cirucuit if we can
        if logical.operator.token_type == TokenType::Or {
            // True or X will alway be True, so if True, then return True
            if left_bool {
                return LiteralType::Boolean(left_bool);
            }
        } else {
            // False AND X will always be False, so return False if is_and && is_false
            if !(left_bool) {
                return LiteralType::Boolean(left_bool);
            }
        }

        //traverse it otherwise
        self.evaluate(&mut logical.right)
    }
    fn visit_call(&mut self, call: &mut Call) -> LiteralType {
        //Taking Ownership here isn't a bad thing because we are decomposing to produce an output,
        //plus the original data is still stored in a file
        let deref = call.to_owned();
        let (paren, mut callee, mut arguments) = (deref.paren, deref.callee, deref.arguments);
        let callee: LiteralType = self.evaluate(&mut callee);
        let error_line = paren.line;
        let mut eval_args = vec![];
        for argument in &mut arguments {
            eval_args.push(self.evaluate(argument));
        }

        let function: Option<Box<dyn CallableTrait>> = match callee {
            LiteralType::Callable(function) => match function {
                Functions::Print(function) => Some(Box::new(function)),
                Functions::Clock(function) => Some(Box::new(function)),
                Functions::User(function) => Some(Box::new(function)),
            },
            _ => None,
        };

        if let Some(func) = function {
            let mut function = func;
            let arity: u64 = function.arity();
            if arity
                != eval_args
                    .len()
                    .try_into()
                    .expect("Expected a length in u64 range")
            {
                crate::error(
                    error_line,
                    format!("Expected {} but got {}", arity, eval_args.len()),
                );
            }
            let result = function.call(self, arguments);
            if let Some(mut to_eval) = result {
                self.evaluate(&mut to_eval)
            } else {
                LiteralType::Nil
            }
        } else {
            LiteralType::Nil
        }
    }
}
