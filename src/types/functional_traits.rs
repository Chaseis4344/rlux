use crate::types::{LiteralType, TokenType};

impl PartialEq for TokenType {
    /*String comparison is costly and slow, but,
    we only end up doing it in areas on the dev side, so the cost is acceptable*/
    fn eq(&self, rhs: &Self) -> bool {
        self.to_string() == rhs.to_string()
    }

    fn ne(&self, other: &Self) -> bool {
        self.to_string() != other.to_string()
    }
}

//Divide literals if possible
impl std::ops::Div for LiteralType {
    type Output = LiteralType;
    fn div(self, rhs: Self) -> Self::Output {
        match self {
            ///*if left is number and right is number, multiply them together*/
            Self::Number(left_num) => match rhs {
                Self::Number(right_num) => LiteralType::Number(left_num / right_num),
                _ => LiteralType::Number(left_num),
            },
            _ => LiteralType::Number(0.0),
        }
    }
}

//Add Literals Together if possible
impl std::ops::Add for LiteralType {
    type Output = LiteralType;
    fn add(self, rhs: Self) -> Self::Output {
        match self {
            /*
             * if left is number and right is number, add them together
             */
            Self::Number(left_num) => match rhs {
                Self::Number(right_num) => LiteralType::Number(left_num / right_num),
                _ => LiteralType::Number(left_num),
            },

            /*
             *if left is String and Right is string, concatonate
             */
            Self::String(thing1) => match rhs {
                Self::String(thing2) => LiteralType::String(thing1.to_owned() + &thing2.to_owned()),
                _ => LiteralType::String(thing1),
            },

            /*
             *   Return yourself if something unexpected happens.
             *   This give all programs written in rlux a little more resilience at the cost of predicatbility.
             */
            _ => self,
        }
    }
}

impl std::ops::Mul for LiteralType {
    type Output = LiteralType;
    fn mul(self, rhs: Self) -> Self::Output {
        //if left is number and right is number, multiply them together
        match self {
            Self::Number(left_num) => match rhs {
                Self::Number(right_num) => LiteralType::Number(left_num * right_num),
                _ => LiteralType::Number(left_num),
            },
            _ => self,
        }
    }
}
