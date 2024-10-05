use crate::types::{token::Token, LiteralType};
use std::{collections::HashMap, env::VarError};

#[derive(Clone)]
pub struct Enviroment {
    pub(crate) enclosing: Box<Option<Enviroment>>,
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

    pub(crate) fn assign(&mut self, name: Token, value: LiteralType) {
        if self.variable_map.contains_key(&name.lexeme) {
            self.variable_map.insert(name.lexeme, value);
        } else if self.enclosing.is_some() {
            self.enclosing.to_owned().unwrap().assign(name, value);
        } else {
            let _ = crate::error(name.line, format!("Undefined Variable {}.", name.lexeme));
        }
    }
}
