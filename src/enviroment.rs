use crate::types::{token::Token, LiteralType};
use std::{collections::HashMap, env::VarError, mem};

#[derive(Clone, Debug)]
pub struct Enviroment {
    pub(crate) enclosing: Option<Box<Enviroment>>,
    pub(crate) variable_map: HashMap<String, LiteralType>,
}

impl Enviroment {
    pub(crate) fn define(&mut self, name: Token, value: LiteralType) {
        let map = &mut self.variable_map;
        map.insert(name.lexeme, value);
    }

    pub(crate) fn get(self, name: Token) -> Result<LiteralType, VarError> {
        let result = self.variable_map.get(&name.lexeme);

        match result {
            Some(lit) => {
                return Ok(lit.to_owned());
            }
            None => {
                if self.enclosing.is_some() {
                    return self.enclosing.unwrap().get(name);
                }
                let _ = crate::error(name.line, "Undefined Variable".to_string());
                return Err(VarError::NotPresent);
            }
        }
    }

    fn swap_define(&mut self, name: Token, value: LiteralType) {
        let mut temp = Enviroment {
            enclosing: None,
            variable_map: HashMap::new(),
        };

        mem::swap(&mut temp, self);

        let map = &mut self.variable_map;
        map.insert(name.lexeme, value);

        mem::swap(&mut temp, self);
    }

    pub(crate) fn assign(&mut self, name: Token, value: LiteralType) {
        if self.variable_map.contains_key(&name.lexeme) {
            self.variable_map.insert(name.lexeme, value);
        } else if self.enclosing.is_some() {
            self.enclosing.as_mut().unwrap().assign(name, value);
        } else {
            let _ = crate::error(name.line, format!("Undefined Variable {}.", name.lexeme));
        }
    }
}
