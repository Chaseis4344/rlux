#![allow(unused_imports, unused_macros)]

macro_rules! debug {
    ($e:expr) => {
        #[cfg(debug_assertions)]
        println!("{:?}", $e);
    };
}

macro_rules! init_value {
    ($default:ident , $val:literal) => {
        let $default = match $default {
            Some(thing) => thing,
            None => $val,
        };
    };
}

///Removes some of Syntactical ugliness in the Scanner
macro_rules! new_character {
    ($token_type:expr, $string:expr, $line:expr) => {
        Some(Token::new($token_type, String::from($string), None, $line))
    };
}

///Removes Syntactical Ugliness in the Parser, and in the scanner
macro_rules! new_literal {
    //Scanner
    ($token_type:expr, $string:expr, $literal_type:expr, $line:expr) => {
        Some(Token::new(
            $token_type,
            String::from($string),
            Some($literal_type),
            $line,
        ))
    };
    //Expressions
    ($value:expr) => {
        Expression::Literal(Box::new(Literal { value: $value }))
    };
}

///Let's me push errors into corrections for the user at runtime, good examples are syntax, etc
macro_rules! error_check {
    ($variable:ident ) => {
        if let Err(ref error) = $variable {
            let _err = crate::error(error.source.line.clone(), error.cause.clone());
            return Err(error.clone());
        }
    };
}

///Internal shorthand to generate Visitor expressions for generating statements
macro_rules! visitable_trait_two_elements {
    ($trait_type1:ty,  $enum_variant:ty, $enum_parent:ty) => {
        impl Visitable<$trait_type1, $enum_parent> for $enum_variant {
            paste::paste! {
                #[doc = "Redirect Visitors to `" $enum_variant "` version."]
                fn accept(&mut self, visitor: &mut $enum_parent) -> $trait_type1 {
                    paste::item! {visitor.[<visit_ $enum_variant:snake:lower>](self)}
                }
            }
        }
    };
}

///Defines helper macro for any boolean operation (Requiring 2 inputs and an operator)
macro_rules! boolean_op {
    ($self:ident, $other:ident, $op:tt) => {
        match $self {
            Self::Number(left_num) => match $other {
                Self::Number(right_num) => *left_num $op *right_num,
                _ =>  {
                    eprintln!("Error: Type Mismatch! \n\tReturned false from a Number while trying to perform: {}" , stringify!($op));
                    false
                },
            },
            _ =>  {
                    eprintln!("Error: Type Mismatch! \n\tReturned false while trying to perform: {}, between: {:?} and {:?}", stringify!($op), $self, $other);
                    false
                },
        }
    };
}

pub(crate) use boolean_op;
pub(crate) use debug;
pub(crate) use error_check;
pub(crate) use init_value;
pub(crate) use new_character;
pub(crate) use new_literal;
pub(crate) use visitable_trait_two_elements;
