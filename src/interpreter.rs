use crate::expression::{Expression};
use crate::token::Token;
use crate::token_type::TokenType;
use crate::value::Value;
use crate::stmt::{ Stmt};
use crate::environment::Environment;
use std::cell::RefCell;
use std::rc::Rc;

use std::io::{self, Write};

#[derive(Debug)]

pub struct RuntimeError {
    pub token: Token, 
    pub message: String,
}

impl RuntimeError {
    pub(crate) fn new(token: &Token, message: &str) -> Self {
        RuntimeError {
            token: token.clone(),
            message: message.to_string()
        }
    }
}

pub struct Interpreter<W: Write> {
    environment: Rc<RefCell<Environment>>,
    out: W,
}

impl Interpreter<io::Stdout> {
    pub fn new() -> Self {
        Self::with_output(io::stdout())
    }
}

impl<W: Write> Interpreter<W> {
    pub fn with_output(out: W) -> Self {
        Interpreter {
            environment: Rc::new(RefCell::new(Environment::new())),
            out
        }
    }
    
    pub fn interpret(&mut self, statements: &[Stmt]) -> Result<(), RuntimeError> {
        for stmt in statements {
            self.execute(stmt)?;
        }
        Ok(())
    }

    fn execute(&mut self, stmt: &Stmt) -> Result<(), RuntimeError> {
        match stmt {
            Stmt::Expression(expr) => {
                self.evaluate(expr)?; // val discarded
            }
            Stmt::Print(expr) => {
                let value = self.evaluate(expr)?;
                writeln!(self.out, "{}", value).expect("failed to write output");
            }
            Stmt::Var {name, initializer} => {
                let value = match initializer {
                    Some(expr) => self.evaluate(expr)?,
                    None => Value::Nil,
                };
                self.environment.borrow_mut().define(name.lexeme.clone(), value);
            }

            Stmt::Block(statements) => {
                let env = Environment::new_enclosed(Rc::clone(&self.environment));
                self.execute_block(statements, Rc::new(RefCell::new(env)))?;
            }

            Stmt::If { condition, then_branch, else_branch } => {
                if Self::is_truthy(&self.evaluate(condition)?) {

                    self.execute(then_branch)?;

                } else if let Some(else_branch) = else_branch {

                    self.execute(else_branch)?;

                }
            }

            Stmt::While { condition, body } => {
                while Self::is_truthy(&self.evaluate(condition)?) {
                    self.execute(body)?;
                }
            }
        }

        Ok(())
    }

    fn evaluate(&mut self, expr: &Expression) -> Result<Value, RuntimeError> {
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

            Expression::Variable { name } => self.environment.borrow().get(name),

            Expression::Assign { name, value } => {
                let var = self.evaluate(value)?;
                self.environment.borrow_mut().assign(name, var.clone())?;
                Ok(var)
            }

             Expression::Logical { left, operator, right } => {
                let l = self.evaluate(left)?;

                if operator.token_type == TokenType::Or {

                    if Self::is_truthy(&l) {
                        return Ok(l);
                    }
                } else if !Self::is_truthy(&l) {
                    return Ok(l);
                }

                self.evaluate(right)
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

    fn execute_block(&mut self, statements: &[Stmt], environment: Rc<RefCell<Environment>>) -> Result<(), RuntimeError> {
        let prev = std::mem::replace(&mut self.environment, environment);
        let result = statements.iter().try_for_each(|stmt| self.execute(stmt));
        self.environment = prev;
        result
    }
}


#[cfg(test)]
mod tests {
    use super::*;
    use crate::parser::Parser;
    use crate::scanner::Scanner;

 
    fn eval(source: &str) -> Result<Value, RuntimeError> {
        let tokens = Scanner::new(format!("{source};")).scan_tokens().clone();
        let stmts = Parser::new(tokens).parse().expect("parse failed");

        match stmts.as_slice() {

            [Stmt::Expression(expression)] => Interpreter::new().evaluate(expression),
            other => panic!("expected one expression statement, got {other:?}"),
        }
    }
 
    fn err(source: &str) -> String {
        eval(source).unwrap_err().message
    }

    fn parse(source: &str) -> Vec<Stmt> {
        let tokens = Scanner::new(source.to_string()).scan_tokens().clone();
        Parser::new(tokens).parse().expect("parse failed")
    }

fn run_then_eval(program: &str, expr: &str) -> Result<Value, RuntimeError> {
    let mut interp = Interpreter::new();

    interp.interpret(&parse(program))?;

    match parse(&format!("{expr};")).as_slice() {

        [Stmt::Expression(e)] => interp.evaluate(e),
        other => panic!("expected one expression statement, got {other:?}"),
    }
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

    #[test]
    fn undefined_variable_is_A_runtime_error() {
        assert_eq!(err("nope"), "Undefined variable 'nope'.");
    }

    #[test]
    fn variables_persist_betweenstatements() {
        assert_eq!(run_then_eval("var a = 1;", "a").unwrap(), Value::Number(1.0));
    }

    #[test]
    fn uninitialized_var_isa_nil() {
        assert_eq!(run_then_eval("var a;", "a").unwrap(), Value::Nil);
    }

    #[test]
    fn globals_can_be_redefined() {
        assert_eq!(
            run_then_eval("var a = 1; var a = \"two\";", "a").unwrap(),
            Value::Str("two".into())
        );
    }

    #[test]
    fn initializer_can_read_other_variables() {
        assert_eq!(run_then_eval("var a = 2; var b = a * 3;", "b").unwrap(), Value::Number(6.0));
    }

    #[test]
    fn undefined_variable_in_initializer_errors() {
        let e = run_then_eval("var b = nope;", "1").unwrap_err();
        assert_eq!(e.message, "Undefined variable 'nope'.");
    }

    #[test]
    fn assignment_updates_variable() {
        assert_eq!(run_then_eval("var a = 1; a = 2;", "a").unwrap(), Value::Number(2.0));
    }

    #[test]
    fn assignment_is_an_expression() {
        assert_eq!(run_then_eval("var a;", "a = 3").unwrap(), Value::Number(3.0));
    }

    #[test]
    fn chained_assignment() {
        assert_eq!(run_then_eval("var a; var b; a = b = 5;", "a").unwrap(), Value::Number(5.0));
    }

    #[test]
    fn assigning_undeclared_variable_errors() {
        assert_eq!(run_then_eval("", "nope = 1").unwrap_err().message, "Undefined variable 'nope'.");
    }

    #[test]
    fn shadowing_does_not_leak() {
        assert_eq!(
            run_then_eval("var a = \"outer\"; { var a = \"inner\"; }", "a").unwrap(),
            Value::Str("outer".into())
        );
    }

    #[test]
    fn assignment_in_block_updates_outer() {
        assert_eq!(run_then_eval("var a = 1; { a = 2; }", "a").unwrap(), Value::Number(2.0));
    }

    #[test]
    fn block_locals_disappear() {
        assert_eq!(
            run_then_eval("{ var b = 1; }", "b").unwrap_err().message,
            "Undefined variable 'b'."
        );
    }

    #[test]
    fn environment_restored_after_error_in_block() {
        let mut interp = Interpreter::new();
        let result = interp.interpret(&parse("var g = \"global\"; { var g = \"local\"; -\"oops\"; }"));
        assert!(result.is_err());
        match parse("g;").as_slice() {
            [Stmt::Expression(e)] => {
                assert_eq!(interp.evaluate(e).unwrap(), Value::Str("global".into()))
            }
            other => panic!("{other:?}"),
        }
    }

    fn run_script(source: &str) -> String {
        let mut interpreter = Interpreter::with_output(Vec::new());
        interpreter.interpret(&parse(source)).expect("runtime error");

        String::from_utf8(interpreter.out).unwrap()
    }

    fn expected_output(source: &str) -> String {
        source
            .lines()
            .filter_map(|line| line.split_once("// expect: "))
            .map(|(_, expected)| format!("{}\n", expected.trim_end()))
            .collect()
    }

    fn check_script(source: &str) {
        assert_eq!(run_script(source), expected_output(source));
    }

    #[test]
    fn logical_returns_operands() {
        check_script(r#"
            print "hi" or 2; // expect: hi
            print nil or "yes"; // expect: yes
            print false and 1; // expect: false
            print 1 and 2; // expect: 2
        "#);
    }

    #[test]
    fn logical_short_circuits() {
        // if the right side ran, a would change
        check_script(r#"
            var a = 0;
            true or (a = 1);
            false and (a = 1);
            print a; // expect: 0
            false or (a = 2);
            print a; // expect: 2
        "#);
    }

    #[test]
    fn if_else_branches() {
        check_script(r#"
            if (true) print "then"; else print "else"; // expect: then
            if (nil) print "then"; else print "else"; // expect: else
            if (false) print "nope";
            if (0) print "zero is truthy"; // expect: zero is truthy
        "#);
    }

    #[test]
    fn dangling_else_runs_correctly() {
        check_script(r#"
            if (true) if (false) print "a"; else print "b"; // expect: b
            if (false) if (true) print "c"; else print "d";
            print "done"; // expect: done
        "#);
    }

    #[test]
    fn while_loop() {
        check_script(r#"
            var i = 0;
            while (i < 3) {
                print i;
                i = i + 1;
            }
            // expect: 0
            // expect: 1
            // expect: 2
        "#);
    }

    #[test]
    fn for_loop_scopes_its_variable() {
        check_script(r#"
            for (var i = 0; i < 2; i = i + 1) print i;
            // expect: 0
            // expect: 1
        "#);
        let e = run_then_eval("for (var i = 0; i < 1; i = i + 1) {}", "i").unwrap_err();
        assert_eq!(e.message, "Undefined variable 'i'.");
    }

    #[test]
    fn for_with_expression_initializer_uses_outer_var() {
        assert_eq!(
            run_then_eval("var i; for (i = 0; i < 5; i = i + 1) {}", "i").unwrap(),
            Value::Number(5.0)
        );
    }

    #[test]
    fn fibonacci_from_the_book() {
        let out = run_script(r#"
            var a = 0;
            var temp;
            for (var b = 1; a < 10000; b = temp + b) {
                print a;
                temp = a;
                a = b;
            }
        "#);
        let nums: Vec<&str> = out.lines().collect();
        assert_eq!(nums.len(), 21);
        assert_eq!(nums.last(), Some(&"6765"));
    }

    #[test]
    fn error_in_while_condition_stops_the_loop() {
        let mut interp = Interpreter::with_output(Vec::new());
        let result = interp.interpret(&parse("while (-\"x\") print 1;"));
        assert_eq!(result.unwrap_err().message, "Operand must be a number.");
    }

}