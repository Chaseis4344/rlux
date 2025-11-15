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
pub struct Enviroment<'env> {
    pub(crate) enclosing: Option<Box<Enviroment<'env>>>,
    pub(crate) variable_map: HashMap<&'env str, LiteralType<'env>>,
}

impl Debug for Enviroment<'_> {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(
            f,
            "Current: \n\tHashMap:{:?}\nNext:\n\t{:?}\n\n",
            self.variable_map, self.enclosing
        )
    }
}

impl<'env> Enviroment<'env> {
    ///Defines a new variable and maps the value to the Literal Provided
    pub(crate) fn define(&mut self, name: &str, value: LiteralType) {
        {
            let map = &mut self.variable_map.to_owned();
            map.insert(name.clone(), value);
        }
        // println!("Enviroment: {:?} defined: {name}",self);
    }

    ///Gets a defined variable, throws a runtime error if non is found
    pub(crate) fn get<'get>(self, name: &'env str) -> Result<LiteralType, VarError> {
        let result = self.variable_map.get(&name);

        if let Some(lit) = result {
            // println!("Gave {lit}");
            return Ok(lit.to_owned());
        }

        if let Some(underlying) = self.enclosing {
            // println!("Enclosing Checked");
            underlying.get(name)
        } else {
            Err(VarError::NotPresent)
        }
    }

    /// Assigns value to variable, may be used to redfine existing varibles
    pub(crate) fn assign<'assign>(&mut self, name: &'env str, value: LiteralType<'env>, line: u32) {
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
