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

    fn ancestor(env: &Rc<RefCell<Environment>>, distance: usize) -> Rc<RefCell<Environment>> {
        let mut current = Rc::clone(env);

        for _ in 0..distance {
            let enclosing = current.borrow().enclosing.clone().expect("resolver said there was an enclosing scope");
            current = enclosing;
        }

        current
    }

    pub fn get_at(env: &Rc<RefCell<Environment>>, distance: usize, name: &Token) -> Value {
        let ancestor = Self::ancestor(env, distance);
        let value = ancestor.borrow().values.get(&name.lexeme).cloned();

        value.expect("resolver said this variable was here")
    }

    pub fn assign_at(env: &Rc<RefCell<Environment>>, distance: usize, name: &Token, value: Value) {

        Self::ancestor(env, distance).borrow_mut().values.insert(name.lexeme.clone(), value);
    } 
}