use crate::types::LiteralType;
use std::{
    collections::HashMap,
    env::VarError,
    fmt::{
        Debug,
        Formatter,
    },
};
#[derive(Clone)]
///Enclosing Enviroment for Rlux runtime
pub struct Enviroment {
    pub(crate) enclosing: Option<Box<Enviroment>>,
    pub(crate) variable_map: HashMap<String, LiteralType>,
}

impl Debug for Enviroment {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(
            f,
            "Current: \n\tHashMap:{:?}\nNext:\n\t{:?}\n\n",
            self.variable_map, self.enclosing
        )
    }
}

impl Enviroment {
    ///Defines a new variable and maps the value to the Literal Provided
    pub(crate) fn define(&mut self, name: &str, value: LiteralType) {
        {
            self.variable_map.insert(name.to_string(), value);
        }
        // println!("Enviroment: {:?} defined: {name}",self);
    }

    ///Gets a defined variable, throws a runtime error if non is found
    pub(crate) fn get(&self, name: &str) -> Result<&LiteralType, VarError> {
        if let Some(lit) = self.variable_map.get(name) {
            // println!("Gave {lit}");
            return Ok(lit);
        }

        if let Some(ref underlying) = self.enclosing {
            // println!("Enclosing Checked");
            underlying.get(name)
        } else {
            Err(VarError::NotPresent)
        }
    }

    /// Assigns value to variable, may be used to redfine existing varibles
    pub(crate) fn assign(&mut self, name: &str, value: LiteralType, line: u32) {
        use std::collections::hash_map::*;

        if let Entry::Occupied(mut entry) = self.variable_map.entry(name.to_string()) {
            entry.insert(value);
        } else if self.enclosing.is_some() {
            self.enclosing.as_mut().unwrap().assign(name, value, line);
        } else {
            crate::error(line, format!("Assignement failed on {name}."));
        }
    }
}
