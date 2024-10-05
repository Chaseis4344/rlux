use crate::types::{LiteralType, TokenType};

macro_rules! number_op {
    ($self:expr, $rhs:expr, $op:tt) => {
        match $self {
            Self::Number(left_num) => match $rhs {
                Self::Number(right_num) => return LiteralType::Number(left_num $op right_num),
                _ => {
                    eprintln!(
                        "Error: Type Mismatch! \n\tReturned \"{}\" from {}!",
                        $self,
                        stringify!($op)
                    );
                    return LiteralType::Number(left_num);
                },
            },
            _ => {
                eprintln!(
                "Error: Type Mismatch! \n\tReturned \"{}\" from {}!",
                $self,
                stringify!($op)
            );
            return $self;
        },
        }

    };
}

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

//Add Literals Together if possible
impl std::ops::Add for LiteralType {
    type Output = LiteralType;
    fn add(self, rhs: Self) -> Self::Output {
        match self {
            /*
             * if left is number and right is number, add them together
             */
            Self::Number(left_num) => {
                match rhs {
                    Self::Number(right_num) => LiteralType::Number(left_num + right_num),
                    _ => {
                        eprintln!("Error: Type Mismatch! \n\tReturned {} from a Number while trying to add!", left_num);
                        LiteralType::Number(left_num)
                    }
                }
            }

            /*
             *if left is String and Right is string, concatonate
             */
            Self::String(left_string) => match rhs {
                Self::String(right_string) => {
                    LiteralType::String(left_string.to_owned() + &right_string.to_owned())
                }
                Self::Boolean(boolean) => LiteralType::String(left_string + &boolean.to_string()),
                Self::Number(num) => LiteralType::String(left_string + &num.to_string()),
                Self::Nil => LiteralType::String(left_string + &Self::Nil.to_string()),
            },

            /*
             *   Return yourself if something unexpected happens.
             *   This give all programs written in rlux a little more resilience at the cost of predicatbility.
             */
            _ => {
                eprintln!(
                    "Error: Type Mismatch! \n\tReturned \"{}\" while trying to add!",
                    self
                );
                self
            }
        }
    }
}

impl std::ops::Sub for LiteralType {
    type Output = LiteralType;
    fn sub(self, rhs: Self) -> Self::Output {
        number_op!(self,rhs, -)
    }
}

impl std::ops::Mul for LiteralType {
    type Output = LiteralType;
    fn mul(self, rhs: Self) -> Self::Output {
        //if left is number and right is number, multiply them together
        number_op!(self,rhs,*)
    }
}

//Divide literals if possible
impl std::ops::Div for LiteralType {
    type Output = LiteralType;
    fn div(self, rhs: Self) -> Self::Output {
        number_op!(self,rhs,/)
    }
}

// ==, !=
impl PartialEq for LiteralType {
    /// For each path we extract both values and directly compare them to one another via Rust
    fn eq(&self, other: &Self) -> bool {
        //How does this all make perfect sense to me?
        //also how did I manage to exhaust all the paths by emulating C?
        match self {
            Self::Boolean(left_boolean) => match other {
                Self::Boolean(right_boolean) => *left_boolean == *right_boolean,
                _ => {
                    /*Type Mismatch*/
                    eprintln!("Error: Type Mismatch! \n\tReturned false from a boolean while trying to check equality!");
                    false
                }
            },
            Self::Number(left_num) => match other {
                Self::Number(right_num) => *left_num == *right_num,
                _ => {
                    /*Type Mismatch*/
                    eprintln!("Error: Type Mismatch! \n\tReturned false from a Number while trying to check equality!");
                    false
                }
            },
            //Rust has String comparison built-in?
            Self::String(left_string) => {
                match other {
                    Self::String(right_string) => *left_string == *right_string,
                    _ => {
                        /*Type Mismatch*/
                        eprintln!("Error: Type Mismatch! \n\tReturned false from a String while trying to check equality!");
                        false
                    }
                }
            }
            //If both are Nil, true else false
            Self::Nil => match other {
                Self::Nil => true,
                _ => false,
            }, //  _ => false, //Error left not number or boolean
        }
    }

    fn ne(&self, other: &Self) -> bool {
        !(self == other)
    }
}

impl From<LiteralType> for bool {
    fn from(value: LiteralType) -> Self {
        match value {
            LiteralType::Boolean(ret) => ret,
            _ => panic!("Coecercing Non-Bool into Bool"),
        }
    }
}

impl From<LiteralType> for f64 {
    fn from(value: LiteralType) -> Self {
        match value {
            LiteralType::Number(number) => number,
            _ => panic!("Coecercing Non-number into Number"),
        }
    }
}

impl From<LiteralType> for String {
    fn from(value: LiteralType) -> Self {
        match value {
            LiteralType::String(stringy) => stringy,
            _ => panic!("Coecercing Non-string into string"),
        }
    }
}

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
                    eprintln!("Error: Type Mismatch! \n\tReturned false while trying to perform: {}", stringify!($op));
                    false
                },
        }
    };
}

//>=, >, <=, <
impl PartialOrd for LiteralType {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
    //Only numbers are ordered
    fn ge(&self, other: &Self) -> bool {
        boolean_op!(self,other, >=)
    }
    fn gt(&self, other: &Self) -> bool {
        boolean_op!(self,other,>)
    }
    fn le(&self, other: &Self) -> bool {
        boolean_op!(self,other, <=)
    }
    fn lt(&self, other: &Self) -> bool {
        boolean_op!(self,other, <)
    }
}

impl Ord for LiteralType {
    fn cmp(&self, other: &LiteralType) -> std::cmp::Ordering {
        //Ripped Striaght from rust's own source
        if *self < *other {
            std::cmp::Ordering::Less
        } else if *self == *other {
            std::cmp::Ordering::Equal
        } else {
            std::cmp::Ordering::Greater
        }
    }
}

//Strange Rust things are Happening
impl Eq for LiteralType {}
