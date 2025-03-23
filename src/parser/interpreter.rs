use super::Statement;
use crate::enviroment::Enviroment;
use crate::types::expression::Call;
use crate::types::expression::*;
use crate::types::lux_functions::{
    clock::Clock, Callable as CallableTrait, Functions::Clock as OuterClock,
};
use crate::types::{Expression, LiteralType, TokenType};
use std::collections::HashMap;

macro_rules! visitable_trait {
    ( $trait_type:ty,$enum_variant:ty, $enum_parent:ty) => {
        impl Visitable<$trait_type> for $enum_variant {
            fn accept(&mut self, visitor: &mut dyn ExpressionVisitor<$trait_type>) -> $trait_type {
                paste::item! {visitor.[<visit_ $enum_variant:lower>](self)}
            }
        }
    };
}

trait ExpressionVisitor<T> {
    fn visit_grouping(&mut self, group: &mut Grouping) -> T;
    fn visit_binary(&mut self, bin: &mut Binary) -> T;
    fn visit_unary(&mut self, unary: &mut Unary) -> T;
    fn visit_literal(&mut self, lit: &mut Literal) -> T;
    fn visit_ternary(&mut self, tern: &mut Ternary) -> T;
    fn visit_variable(&mut self, var: &mut Variable) -> T;
    fn visit_assignment(&mut self, assign: &mut Assignment) -> T;
    fn visit_logical(&mut self, logical: &mut Logical) -> T;
    fn visit_call(&mut self, call: &mut Call) -> T;
}

trait Visitable<T> {
    fn accept(&mut self, visitor: &mut dyn ExpressionVisitor<T>) -> T;
}

impl Visitable<LiteralType> for Expression {
    fn accept(&mut self, visitor: &mut dyn ExpressionVisitor<LiteralType>) -> LiteralType {
        match self {
            Expression::Binary(bin) => bin.accept(visitor),
            Expression::Literal(lit) => lit.accept(visitor),
            Expression::Grouping(group) => group.accept(visitor),
            Expression::Unary(unary) => unary.accept(visitor),
            Expression::Ternary(tern) => tern.accept(visitor),
            Expression::Variable(var) => var.accept(visitor),
            Expression::Assignment(assign) => assign.accept(visitor),
            Expression::Logical(logic) => logic.accept(visitor),
            Expression::Call(call) => call.accept(visitor),
        }
    }
}

visitable_trait! {LiteralType,Binary,Expression}
visitable_trait! {LiteralType,Literal,Expression}
visitable_trait! {LiteralType,Grouping,Expression}
visitable_trait! {LiteralType,Unary,Expression}
visitable_trait! {LiteralType,Ternary,Expression}
visitable_trait! {LiteralType,Variable,Expression}
visitable_trait! {LiteralType,Assignment,Expression}
visitable_trait! {LiteralType,Logical,Expression}
visitable_trait! {LiteralType,Call,Expression}

pub(crate) struct Interpreter {
    pub(crate) enviroment: Box<Enviroment>,
    pub(crate) globals: Enviroment,
}

// fun -> LiteralType | fun

impl Interpreter {
    pub(crate) fn evaluate(&mut self, expr: &mut Expression) -> LiteralType {
        expr.accept(self)
    }
    pub(crate) fn new() -> Interpreter {
        let map = HashMap::new();
        let mut globals = Enviroment {
            enclosing: None,
            variable_map: map,
        };
        let clock = OuterClock(Clock {});

        globals.define(String::from("clock"),LiteralType::Callable(clock));
        let enviroment = Box::new(globals.clone());
        Interpreter {
            enviroment,
            globals,
        }
    }
    pub(crate) fn interpret(&mut self, statements: Vec<Statement>) {
        for statement in statements {
            self.execute(statement);
        }
    }

    pub(crate) fn execute(&mut self, mut statement: Statement) {
        use super::statement::Visitable as InterpreterVisitable;
        //Hand over between the two architectures
        statement.accept(self);
    }

    pub(crate) fn execute_block(&mut self, statements: Vec<Statement>) {
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
    }
}

///Logic for how the Interpreter acts with each Data Type
impl ExpressionVisitor<LiteralType> for Interpreter {
    fn visit_binary(&mut self, bin: &mut Binary) -> LiteralType {
        let left = self.evaluate(&mut bin.left);
        let right = self.evaluate(&mut bin.right);
        let operator = &bin.operator;

        //We can abstract all this logic away to rust's traits
        /*TODO: ARCHITECT WAY FOR TYPE ERRORS TO BE PASSED UP FROM HERE TO USER */
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
            _ => left,
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
        let mut left = &mut tern.left;
        let mut right = &mut tern.right;

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
            _ => right,
        }
    }

    fn visit_variable(&mut self, var: &mut Variable) -> LiteralType {
        let result = self.enviroment.to_owned().get(var.to_owned().name.lexeme);
        if let Ok(item) = result {
            item
        } else {
            LiteralType::Nil
        }
    }

    fn visit_assignment(&mut self, assign: &mut Assignment) -> LiteralType {
        //Decompose assignment to avoid excess cloning
        let (name, value) = (assign.name.to_owned(), &mut assign.value);

        //Evaluate expression inside
        let value = self.evaluate(value);

        //Copy the value then echo out for the rest of the syntax tress
        self.enviroment.assign(name.lexeme, value.clone());

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
        for mut argument in &mut arguments {
            eval_args.push(self.evaluate(argument));
        }

        let function: Option< &mut dyn CallableTrait> = match callee {
            LiteralType::Callable(ref func) => todo!(),
            _ => {
                crate::error(
                    error_line,
                    String::from("Cannot call a non-callable function"),
                );
                None
            }
        };

        if function.is_some() {
            let mut function = function.expect("Expected a function");
            let arity:u64 = function.arity();
            if arity
                != eval_args
                    .len()
                    .try_into()
                    .expect("Expected a length in u64 range")
            {
                _ = crate::error(
                    error_line,
                    format!("Expected {} but got {}", arity, eval_args.len()),
                );
            }
            let mut result = function.call(self, arguments);
            self.evaluate(&mut result)
        } else {
            LiteralType::Nil
        }
    }
}
