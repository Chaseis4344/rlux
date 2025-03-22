use crate::types::{token::Token, LiteralType};
use std::{collections::HashMap, env::VarError};

#[derive(Clone, Debug)]
///Enclosing Enviroment for Rlux runtime
pub struct Enviroment {
    pub(crate) enclosing: Option<Box<Enviroment>>,
    pub(crate) variable_map: HashMap<String, LiteralType>,
}

impl Enviroment {
    ///Defines a new variable and maps the value to the Literal Provided
    pub(crate) fn define(&mut self, name: Token, value: LiteralType) {
        let map = &mut self.variable_map;
        map.insert(name.lexeme, value);
    }

    ///Gets a defined variable, throws a runtime error if non is found
    pub(crate) fn get(self, name: Token) -> Result<LiteralType, VarError> {
        let result = self.variable_map.get(&name.lexeme);

        match result {
            Some(lit) => Ok(lit.to_owned()),
            None => {
                if self.enclosing.is_some() {
                    return self.enclosing.unwrap().get(name);
                }
                let _ = crate::error(name.line, "Undefined Variable".to_string());
                Err(VarError::NotPresent)
            }
        }
    }

    /// Assigns value to variable, may be used to redfine existing varibles
    pub(crate) fn assign(&mut self, name: Token, value: LiteralType) {
        if let std::collections::hash_map::Entry::Occupied(mut entry) =
            self.variable_map.entry(name.lexeme.clone())
        {
            entry.insert(value);
        } else if self.enclosing.is_some() {
            self.enclosing.as_mut().unwrap().assign(name, value);
        } else {
            let _ = crate::error(name.line, format!("Undefined Variable {}.", name.lexeme));
        }
    }
}
