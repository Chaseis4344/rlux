use crate::types::LiteralType;
use std::{collections::HashMap, env::VarError};

#[derive(Clone, Debug)]
///Enclosing Enviroment for Rlux runtime
pub struct Enviroment {
    pub(crate) enclosing: Option<Box<Enviroment>>,
    pub(crate) variable_map: HashMap<String, LiteralType>,
}

impl Enviroment {
    ///Defines a new variable and maps the value to the Literal Provided
    pub(crate) fn define(&mut self, name: String, value: LiteralType) {
        let map = &mut self.variable_map;
        map.insert(name, value);
    }

    ///Gets a defined variable, throws a runtime error if non is found
    pub(crate) fn get(self, name: String) -> Result<LiteralType, VarError> {
        let result = self.variable_map.get(&name);

        match result {
            Some(lit) => Ok(lit.to_owned()),
            None => {
                if let Some(underlying) = self.enclosing {
                    underlying.get(name)
                } else {
                    Err(VarError::NotPresent)
                }
            }
        }
    }

    /// Assigns value to variable, may be used to redfine existing varibles
    pub(crate) fn assign(&mut self, name: String, value: LiteralType, line: u32) {
        use std::collections::hash_map::*;

        if let Entry::Occupied(mut entry) = self.variable_map.entry(name.clone()) {
            entry.insert(value);
        } else if self.enclosing.is_some() {
            self.enclosing.as_mut().unwrap().assign(name, value, line);
        } else {
            let _ = crate::error(line, format!("Assignement failed on {}.", name));
        }
    }
}
