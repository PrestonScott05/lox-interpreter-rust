use crate::token::Literal;
use std::fmt;
use crate::environment::Environment;
use crate::stmt::FunctionDecl;
use std::cell::RefCell;
use std::rc::Rc;


#[derive(Debug, Clone, PartialEq)]
pub enum Value {
    Number(f64),
    Str(String),
    Bool(bool),
    Nil,
    NativeFunction(Rc<NativeFunction>),
    Function(Rc<LoxFunction>),
}

#[derive(Debug)]
pub struct NativeFunction {
    pub arity: usize,
    pub func: fn(&[Value]) -> Value,
}

pub struct LoxFunction {
    pub declaration: Rc<FunctionDecl>,
    pub closure: Rc<RefCell<Environment>>,
}

//we have to do this st the functions are only eq if they're literally the same object
impl PartialEq for NativeFunction {
    fn eq(&self, other: &Self) -> bool {
        std::ptr::eq(self, other)
    }
}

impl PartialEq for LoxFunction {
    fn eq(&self, other: &Self) -> bool {
        std::ptr::eq(self, other)
    }
}

impl fmt::Debug for LoxFunction {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "<fn {}>", self.declaration.name.lexeme)
    }
}

impl From<&Literal> for Value {
    fn from(lit: &Literal) -> Self {
        match lit {
            Literal::Number(n) => Value::Number(*n),
            Literal::Str(s) => Value::Str(s.clone()),
            Literal::Bool(b) => Value::Bool(*b),
            Literal::Nil => Value::Nil,
            
        }
    }
}

impl fmt::Display for Value {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Value::Number(n) => write!(f, "{}", stringify_number(*n)),
            Value::Str(s) => write!(f, "{}", s),
            Value::Bool(b) => write!(f, "{}", b),
            Value::Nil => write!(f, "nil"),
            Value::NativeFunction(_) => write!(f, "<native fn>"),
            Value::Function(func) => write!(f, "<fn {}>", func.declaration.name.lexeme)
        }
    }
}


fn stringify_number(n: f64) -> String {
    if n.is_nan() {
        return "NaN".to_string();
    }
    if n.is_infinite() {
        return if n > 0.0 { "Infinity" } else { "-Infinity" }.to_string();
    }

    let abs = n.abs();
    let text = if abs != 0.0 && (abs < 1e-3 || abs >= 1e7) {
     
        let sci = format!("{:e}", n);
        let (mantissa, exp) = sci.split_once('e').unwrap();
        let mantissa = if mantissa.contains('.') { mantissa.to_string() } else { format!("{}.0", mantissa) };
        format!("{}E{}", mantissa, exp)
    } else {
       
        format!("{}", n)
    };

    // stringify only strips ".0" at the very end, so "1.0E21" stays as is
    text.strip_suffix(".0").map(str::to_string).unwrap_or(text)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn show(n: f64) -> String {
        Value::Number(n).to_string()
    }

    #[test]
    fn matches_jlox_number_output() {
        assert_eq!(show(3.0), "3");
        assert_eq!(show(-0.0), "-0");
        assert_eq!(show(0.5), "0.5");
        assert_eq!(show(1.0 / 0.0), "Infinity");
        assert_eq!(show(-1.0 / 0.0), "-Infinity");
        assert_eq!(show(f64::NAN), "NaN");
        assert_eq!(show(1e21), "1.0E21");
        assert_eq!(show(12345678.0), "1.2345678E7");
        assert_eq!(show(9999999.0), "9999999");
        assert_eq!(show(0.001), "0.001");
        assert_eq!(show(0.0001), "1.0E-4");
        assert_eq!(show(-1.5e-5), "-1.5E-5");
    }
}