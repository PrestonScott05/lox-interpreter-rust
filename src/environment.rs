use std::cell::RefCell;
use std::rc::Rc;

use std::collections::HashMap;
use crate::interpreter::RuntimeError;
use crate::token::Token;
use crate::value::Value;

pub struct Environment {
    values: HashMap<String, Value>,
    enclosing: Option<Rc<RefCell<Environment>>>,
}

impl Environment {
    pub fn new() -> Self {
        Environment {values: HashMap::new(), enclosing: None
        }
    }

    pub fn new_enclosed(enclosing: Rc<RefCell<Environment>>) -> Self {
        Environment {
            values: HashMap::new(),
            enclosing: Some(enclosing)
        }
    }

    pub fn define(&mut self, name: String, value: Value) {
        self.values.insert(name, value); //this overrites because we allow redefinition
    }

    pub fn get(&self, name: &Token) -> Result<Value, RuntimeError> {
        if let Some(var) = self.values.get(&name.lexeme) {
            return Ok(var.clone());
        }
        match &self.enclosing {
            Some(enclosing) => enclosing.borrow().get(name),
            None => Err(RuntimeError::new(
                name, 
                &format!("Undefined variable '{}'.", name.lexeme),
            ))
        }
    } 

    pub fn assign(&mut self, name: &Token, value: Value) -> Result<(), RuntimeError> {
        if let Some(placeholder) = self.values.get_mut(&name.lexeme) {
            *placeholder = value;
            return Ok(());
        }

        match &self.enclosing {
            Some(enclosing) => enclosing.borrow_mut().assign(name, value),
            None => Err(RuntimeError::new(
                name, 
                &format!("Undefined variable '{}'.", name.lexeme),
            ))
        }
    }
}