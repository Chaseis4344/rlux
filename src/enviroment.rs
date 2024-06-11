use crate::types::{token::Token, LiteralType};
use std::{collections::HashMap, env::VarError};

#[derive(Clone)]
pub struct Enviroment {
    pub(crate) variable_map: HashMap<String, LiteralType>,
}

impl Enviroment {
    pub(crate) fn define(&mut self, name: Token, value: LiteralType) {
        self.variable_map.insert(name.lexeme, value);
    }

    pub(crate) fn get(self, name: Token) -> Result<LiteralType, VarError> {
        let result = self.variable_map.get(&name.lexeme);

        if result.is_some() {
            return Ok(result.unwrap().clone());
        } else {
            crate::error(1, "Error Undefined Variable".to_string());
            Err(VarError::NotPresent)
        }
    }
}
