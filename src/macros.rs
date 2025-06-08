#![allow(unused_imports, unused_macros)]
macro_rules! init_value {
    ($default:ident , $val:literal) => {
        let $default = match $default {
            Some(thing) => thing,
            None => $val,
        };
    };
}

macro_rules! new_character {
    ($token_type:expr, $string:expr, $line:expr) => {
        Some(Token::new($token_type, String::from($string), None, $line))
    };
}

macro_rules! new_literal {
    ($token_type:expr, $string:expr, $literal_type:expr, $line:expr) => {
        Some(Token::new(
            $token_type,
            String::from($string), 
            Some($literal_type),
            $line,
        ))
    };
}

///Let's me push errors into corrections for the user at runtime, good examples are syntax, etc
macro_rules! error_check {
    ($variable:ident ) => (
    if let Err(ref error) = $variable {
        let _ = crate::error(error.source.line,error.cause.clone());
        }
    )
}

pub(crate) use error_check;
pub(crate) use init_value;
pub(crate) use new_character;
pub(crate) use new_literal;

