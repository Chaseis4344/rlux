use crate::types::{LiteralType, TokenType};

impl PartialEq for TokenType {
    fn eq(&self, rhs: &Self) -> bool {
        self.to_string() == rhs.to_string()
    }

    fn ne(&self, other: &Self) -> bool {
        self.to_string() != other.to_string()
    }
}

impl std::ops::Div for LiteralType {
    type Output = LiteralType;
    fn div(self, rhs: Self) -> Self::Output {
        match self {
            Self::Number(left_num) => match rhs {
                Self::Number(right_num) => LiteralType::Number(left_num / right_num),
                _ => LiteralType::Number(left_num),
            },
            _ => LiteralType::Number(0.0),
        }
    }
}

impl std::ops::Add for LiteralType {
    type Output = LiteralType;
    fn add(self, rhs: Self) -> Self::Output {
        match self {
            Self::Number(left_num) => match rhs {
                Self::Number(right_num) => LiteralType::Number(left_num / right_num),
                _ => LiteralType::Number(left_num),
            },
            Self::String(thing1) => match rhs {
                Self::String(thing2) => LiteralType::String(thing1.to_owned() + &thing2.to_owned()),
                _ => LiteralType::String(thing1),
            },
            _ => self,
        }
    }
}

impl std::ops::Mul for LiteralType {
    type Output = LiteralType;
    fn mul(self, rhs: Self) -> Self::Output {
        match self {
            Self::Number(left_num) => match rhs {
                Self::Number(right_num) => LiteralType::Number(left_num * right_num),
                _ => LiteralType::Number(left_num),
            },
            _ => self,
        }
    }
}
