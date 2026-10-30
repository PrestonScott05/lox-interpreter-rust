use std::unreachable;

use crate::expression::Expression;
use crate::token::Token;
use crate::token_type::TokenType;
use crate::value::Value;

#[derive(Debug)]

pub struct RuntimeError {
    pub token: Token, 
    pub message: String,
}

impl RuntimeError {
    fn new(token: &Token, message: &str) -> Self {
        RuntimeError {
            token: token.clone(),
            message: message.to_string()
        }
    }
}

pub struct Interpreter;

impl Interpreter {
    pub fn new() -> Self {
        Interpreter
    }
    
    pub fn interpret(&self, expr: &Expression) -> Result<(), RuntimeError> {
        let value = self.evaluate(expr)?;
        println!("{}", value);
        Ok(())
    }

    fn evaluate(&self, expr: &Expression) -> Result<Value, RuntimeError> {
        match expr {
            Expression::Literal { value } => Ok(Value::from(value)),


            
            Expression::Grouping {expression} => self.evaluate(expression),

            Expression::Unary {operator, right} => {
                let r = self.evaluate(right)?;

                match(operator.token_type, r) {
                    (TokenType::Minus, Value::Number(n)) => Ok(Value::Number(-n)),

                    (TokenType::Minus, _ ) => Err(RuntimeError::new(operator, "Operand must be a number.")),

                    (TokenType::Bang, r) => Ok(Value::Bool(!Self::is_truthy(&r))),

                    _ => unreachable!("parser only produces - and ! as unary operators"),
                }
            }

            Expression::Binary { left, operator, right } => {
                let l = self.evaluate(left)?;
                let r = self.evaluate(right)?;

                use TokenType as T;
                use Value::{Bool, Number, Str};
                match (operator.token_type, l, r) {
                    (T::Plus, Number(a), Number(b)) => Ok(Number(a + b)),

                    (T::Plus, Str(a), Str(b)) => Ok(Str(a + &b)),
                    
                    (T::Plus, _, _) => Err(RuntimeError::new(
                        operator,
                        "Operands must be two numbers or two strings.",
                    )),

                    (T::Minus, Number(a), Number(b)) => Ok(Number(a - b)),

                    (T::Star, Number(a), Number(b)) => Ok(Number(a * b)),

                    (T::Slash, Number(a), Number(b)) => Ok(Number(a / b)),

                    (T::Greater, Number(a), Number(b)) => Ok(Bool(a > b)),

                    (T::GreaterEqual, Number(a), Number(b)) => Ok(Bool(a >= b)),

                    (T::Less, Number(a), Number(b)) => Ok(Bool(a < b)),

                    (T::LessEqual, Number(a), Number(b)) => Ok(Bool(a <= b)),
                    (
                        T::Minus | T::Star | T::Slash | T::Greater | T::GreaterEqual | T::Less | T::LessEqual,
                        _,
                        _,
                    ) => Err(RuntimeError::new(operator, "Operands must be numbers.")),
 
                    (T::EqualEqual, a, b) => Ok(Bool(Self::is_equal(&a, &b))),
                    (T::BangEqual, a, b) => Ok(Bool(!Self::is_equal(&a, &b))),
 
                    _ => unreachable!("parser only produces arithmetic/comparison/equality binary operators"),
                }
            }
        }
    }

    fn is_truthy(value: &Value) -> bool {
        !matches!(value, Value::Nil | Value::Bool(false))
    }

    fn is_equal(a: &Value, b: &Value) -> bool {
        match (a, b) {
            (Value::Number(x), Value::Number(y)) if x.is_nan() && y.is_nan() => true,
            _ => a == b,
        }
    }
}


#[cfg(test)]
mod tests {
    use super::*;
    use crate::parser::Parser;
    use crate::scanner::Scanner;

 
    fn eval(source: &str) -> Result<Value, RuntimeError> {
        let tokens = Scanner::new(source.to_string()).scan_tokens().clone();
        let expr = Parser::new(tokens).parse().expect("parse failed");


        Interpreter::new().evaluate(&expr)
    }
 
    fn err(source: &str) -> String {
        eval(source).unwrap_err().message
    }
 
    #[test]
    fn arithmetic_and_precedence() {

        assert_eq!(eval("(1 + 2) * 3 - 4 / 2").unwrap(), Value::Number(7.0));
        assert_eq!(eval("-(3)").unwrap(), Value::Number(-3.0));
    }
 
    #[test]
    fn string_concat() {
        assert_eq!(eval("\"a\" + \"b\"").unwrap(), Value::Str("ab".into()));
    }
 
    #[test]
    fn truthiness() {

        assert_eq!(eval("!nil").unwrap(), Value::Bool(true));

        assert_eq!(eval("!0").unwrap(), Value::Bool(false));
        assert_eq!(eval("!\"\"").unwrap(), Value::Bool(false));

    }
 
    #[test]
    fn equality() {
        assert_eq!(eval("nil == nil").unwrap(), Value::Bool(true));

        assert_eq!(eval("3 == \"3\"").unwrap(), Value::Bool(false));
        assert_eq!(eval("1 != 2").unwrap(), Value::Bool(true));
    }
 
    #[test]
    fn comparison() {
        assert_eq!(eval("2 >= 2").unwrap(), Value::Bool(true));
        assert_eq!(eval("1 > 2").unwrap(), Value::Bool(false));
    }
 
    #[test]
    fn runtime_errors() {
        assert_eq!(err("-\"muffin\""), "Operand must be a number.");
        assert_eq!(err("1 + \"a\""), "Operands must be two numbers or two strings.");
        assert_eq!(err("1 < true"), "Operands must be numbers.");
        assert_eq!(err("2 * (3 / -\"muffin\")"), "Operand must be a number.");
    }
 
    #[test]
    fn nan_equals_itself_like_jlox() {
        assert_eq!(eval("(0 / 0) == (0 / 0)").unwrap(), Value::Bool(true));
    }
 
    #[test]
    fn display_strips_trailing_zero() {
        assert_eq!(eval("1 + 2").unwrap().to_string(), "3");
        assert_eq!(eval("1 / 2").unwrap().to_string(), "0.5");
    }
}