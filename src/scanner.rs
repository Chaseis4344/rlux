use crate::types::{token::Token, LiteralType, TokenType};
pub struct Scanner {
    source: String,
    current: u32,
    pub line: u32,
}

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

#[allow(clippy::manual_range_contains)]
impl Scanner {
    pub fn new(source: String, current: Option<u32>, line: Option<u32>) -> Scanner {
        init_value!(current, 0);
        init_value!(line, 1);

        Scanner {
            source,
            current,
            line,
        }
    }

    //Advance the cursor then return resulting token
    fn scan_token(&mut self) -> Option<Token> {
        //Basically a shitty hashmap, but too much time invested to change
        match self.advance() {
            ' ' | '\t' | '\r' => {
                /* White Space goes here*/
                None
            }
            '\n' => {
                self.line += 1;
                None
            }
            '/' => {
                if self.peek() == '*' {
                    self.multi_line_comment();
                    //Return no token
                    None
                } else if self.peek() == '/' {
                    self.single_line_comment();
                    None
                } else {
                    //Literal Slash
                    new_character!(TokenType::Slash, "/", self.line)
                }
            }
            '(' => new_character!(TokenType::LeftParen, "(", self.line),
            ')' => new_character!(TokenType::RightParen, ")", self.line),
            '{' => new_character!(TokenType::LeftBrace, "{", self.line),
            '}' => new_character!(TokenType::RightBrace, "}", self.line),
            '.' => new_character!(TokenType::Dot, ".", self.line),
            '-' => new_character!(TokenType::Minus, "-", self.line),
            '+' => new_character!(TokenType::Plus, "+", self.line),
            ';' => new_character!(TokenType::Semicolon, ";", self.line),
            '*' => new_character!(TokenType::Star, "*", self.line),
            '?' => new_character!(TokenType::Question, "?", self.line),
            ':' => new_character!(TokenType::Colon, ":", self.line),
            ',' => new_character!(TokenType::Comma, ",", self.line),
            '!' => {
                if self.peek() == '=' {
                    //Discard and return combo character
                    let _ = self.advance();
                    new_character!(TokenType::BangEqual, "!=", self.line)
                } else {
                    new_character!(TokenType::Bang, "!", self.line)
                }
            }
            '=' => {
                if self.peek() == '=' {
                    //Discard and return combo character
                    let _ = self.advance();
                    new_character!(TokenType::EqualEqual, "==", self.line)
                } else {
                    new_character!(TokenType::Equal, "=", self.line)
                }
            }
            '<' => {
                if self.peek() == '=' {
                    //Discard and return combo character
                    let _ = self.advance();
                    new_character!(TokenType::LessEqual, "<=", self.line)
                } else {
                    new_character!(TokenType::Less, "<", self.line)
                }
            }
            '>' => {
                if self.peek() == '=' {
                    //Discard and return combo character
                    let _ = self.advance();
                    new_character!(TokenType::GreaterEqual, ">=", self.line)
                } else {
                    new_character!(TokenType::Greater, ">", self.line)
                }
            }

            'a'..='z' | 'A'..='Z' | '_' => self.keywords(),

            '0'..='9' => self.numbers(),

            '"' => self.strings(),
            chara => {
                let _ = crate::error(self.line, format!("Unexpected Token: '{}'", chara));
                None
            }
        }
    }

    fn multi_line_comment(&mut self) {
        //Run through comment
        let mut in_comment = true;
        let mut current_char = self.advance();
        while in_comment {
            if current_char == '*' && self.peek() == '/' {
                in_comment = false;
            } else {
                current_char = self.advance();
                if current_char == '\n' {
                    self.line += 1;
                }
            }
        }
        self.advance();
    }

    fn single_line_comment(&mut self) {
        let mut in_comment = true;
        let _ = self.advance();
        while in_comment {
            let current_char = self.advance();
            if current_char == '\n' {
                in_comment = false;
            }
        }
        self.line += 1;
    }

    fn strings(&mut self) -> Option<Token> {
        let _in_string = true;
        let mut result = String::from("");
        let mut current_char: char;
        //while in_string
        loop {
            current_char = self.advance();

            //Escape Charaters & Endings
            if current_char == '\\' {
                let next_char = self.peek();
                match next_char {
                    '"' | 'n' => {
                        result.push(next_char);
                        //Over the Literal "
                        self.advance();
                        continue;
                    }

                    _ => {}
                }
            }
            if (current_char == '"') && !self.is_at_end() {
                //in_string = false;
                break;
            }

            if current_char == '\n' {
                self.line += 1;
            }
            if self.is_at_end() {
                let _ = crate::error(self.line, String::from("Unterminated String"));
                // in_string = false;
                return None;
            }

            //Add to result
            result.push(current_char);
        }

        new_literal!(
            TokenType::String,
            result.to_owned(),
            LiteralType::String(result),
            self.line
        )
    }

    fn is_ascii_ident(ch: char) -> bool {
        (ch >= 'a' && ch <= 'z')
            || (ch >= 'A' && ch <= 'Z')
            || (ch >= '0' && ch <= '9')
            || ch == '_'
    }

    fn keywords(&mut self) -> Option<Token> {
        let mut current_char: char = self.source.as_bytes()[(self.current - 1) as usize] as char;
        let mut word_built = String::from("");

        //Read in full word
        while Self::is_ascii_ident(current_char) && (self.current <= self.source.len() as u32) {
            word_built.push(current_char);
            self.current += 1;
            current_char = self.source.as_bytes()[(self.current - 1) as usize] as char
        }
        self.current -= 1;
        let matching = word_built.to_ascii_lowercase();
        //Fat Match lol - Match to keywords
        match matching.as_str() {
            "and" => new_character!(TokenType::And, word_built.as_str(), self.line),
            "class" => new_character!(TokenType::Class, word_built.as_str(), self.line),
            "else" => new_character!(TokenType::Else, word_built.as_str(), self.line),
            "fun" => new_character!(TokenType::Fun, word_built.as_str(), self.line),
            "for" => new_character!(TokenType::For, word_built.as_str(), self.line),
            "if" => new_character!(TokenType::If, word_built.as_str(), self.line),
            "or" => new_character!(TokenType::Or, word_built.as_str(), self.line),
            // "print" => new_character!(TokenType::Print, word_built.as_str(), self.line),
            "return" => new_character!(TokenType::Return, word_built.as_str(), self.line),
            "super" => new_character!(TokenType::Super, word_built.as_str(), self.line),
            "this" => new_character!(TokenType::This, word_built.as_str(), self.line),
            "var" => new_character!(TokenType::Var, word_built.as_str(), self.line),
            "while" => new_character!(TokenType::While, word_built.as_str(), self.line),
            "nil" => new_literal!(
                TokenType::Nil,
                word_built.as_str(),
                LiteralType::Nil,
                self.line
            ),
            "false" => new_literal!(
                TokenType::False,
                word_built.as_str(),
                LiteralType::Boolean(false),
                self.line
            ),
            "true" => new_literal!(
                TokenType::True,
                word_built.as_str(),
                LiteralType::Boolean(true),
                self.line
            ),
            _ => new_character!(TokenType::Identifier, word_built, self.line),
        }
    }

    fn numbers(&mut self) -> Option<Token> {
        let mut current_char: char = self.source.as_bytes()[(self.current - 1) as usize] as char;
        let mut result_string: String = String::from("");

        while is_ascii_num(current_char) {
            result_string.push(current_char);
            current_char = self.advance();
        }

        //Decimal Stuff
        if current_char == '.' && is_ascii_num(self.peek()) {
            result_string.push('.');
            current_char = self.advance();
            while is_ascii_num(current_char) {
                result_string.push(current_char);
                current_char = self.advance();
            }
        }

        //Skip the dot at the end, just in case the user slips up / forgets
        if current_char == '.' {
            self.current += 1;
        }

        //Undiscard next character if we have not used it
        if !is_ascii_num(current_char) {
            self.current -= 1;
        }

        new_literal!(
            TokenType::Number,
            result_string.to_owned(),
            LiteralType::Number(result_string.parse::<f64>().unwrap()),
            self.line
        )
    }

    fn is_at_end(&self) -> bool {
        self.current >= self.source.len().try_into().unwrap()
    }

    fn advance(&mut self) -> char {
        if !self.is_at_end() {
            self.current += 1;
            self.source.as_bytes()[(self.current - 1) as usize] as char
        } else {
            '\0'
        }
    }

    fn peek(&self) -> char {
        self.source.as_bytes()[(self.current) as usize] as char
    }
    //Extract tokens from source
    pub(crate) fn scan_tokens(&mut self) -> Vec<Token> {
        let mut tokens: Vec<Token> = vec![];
        //let mut current_line = 0;

        while !self.is_at_end() {
            //let current_char: char = self.source.as_bytes()[i] as char;
            //let next_char: char = self.source.as_bytes()[i + 1] as char;
            if let Some(current_token) = self.scan_token() {
                tokens.push(current_token);
            };
        }
        tokens
    }
}

#[allow(clippy::manual_range_contains)]
pub(crate) fn is_ascii_num(ch: char) -> bool {
    ch >= '0' && ch <= '9'
}
