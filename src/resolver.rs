use std::collections::HashMap;
use std::fmt;

use crate::expression::Expression;
use crate::stmt::{FunctionDecl, Stmt};
use crate::token::Token;

#[derive(Debug)]
pub struct ResolveError {
    pub token: Token,
    pub message: String,
}


impl fmt::Display for ResolveError {
    
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "[line {}] Error at '{}': {}", self.token.line, self.token.lexeme, self.message)
    }

}

#[derive(Debug, Clone, Copy, PartialEq)]
enum FunctionType {
    None,
    Function,
    Initializer,
    Method,
}

#[derive(Debug, Clone, Copy, PartialEq)]
enum ClassType {
    None,
    Class,
}

//thiswalks the tree once before running it, and figures out how many scopes away each local variable lives.
//globals are left out of the map bc the interpreter looks those up dynamically
pub struct Resolver {
    scopes: Vec<HashMap<String, bool>>,
    locals: HashMap<usize, usize>,
    current_function: FunctionType,
    current_class: ClassType,
    errors: Vec<ResolveError>,
}

impl Resolver {
    pub fn new() -> Self {
        Resolver {
            scopes: Vec::new(),
            locals: HashMap::new(),
            current_function: FunctionType::None,
            current_class: ClassType::None,
            errors: Vec::new(),
        }
    }

    pub fn resolve(mut self, statements: &[Stmt]) -> Result<HashMap<usize, usize>, Vec<ResolveError>> {
        
        self.resolve_stmts(statements);

        if self.errors.is_empty() {

            Ok(self.locals)
        } else {
            Err(self.errors)
        }
    }

    fn resolve_stmts(&mut self, statements: &[Stmt]) {
        
        for stmt in statements {

            self.resolve_stmt(stmt);

        }
    }

    fn resolve_stmt(&mut self, stmt: &Stmt) {
        match stmt {

            Stmt::Block(statements) => {
                self.begin_scope();
                self.resolve_stmts(statements);
                self.end_scope();
            }

            Stmt::Var { name, initializer } => {
    
                self.declare(name);

                if let Some(init) = initializer {
                    self.resolve_expr(init);
                }

                self.define(name);
            }

            Stmt::Function(decl) => {
                
                self.declare(&decl.name);
                
                self.define(&decl.name);
                
                self.resolve_function(decl, FunctionType::Function);
            }

            Stmt::Class { name, methods } => {
                let enclosing_class = self.current_class;
                self.current_class = ClassType::Class;

                self.declare(name);
                self.define(name);

                self.begin_scope();
                if let Some(scope) = self.scopes.last_mut() {
                    scope.insert("this".to_string(), true);
                }

                for method in methods {
                    let kind = if method.name.lexeme == "init" {
                        FunctionType::Initializer
                    } else {
                        FunctionType::Method
                    };
                    self.resolve_function(method, kind)
                }

                self.end_scope();
                self.current_class = enclosing_class;
            }

            Stmt::Expression(expr) | Stmt::Print(expr) => self.resolve_expr(expr),

            Stmt::If { condition, then_branch, else_branch } => {
                self.resolve_expr(condition);
                
                self.resolve_stmt(then_branch);
                
                if let Some(else_branch) = else_branch {
                    
                    self.resolve_stmt(else_branch);
                
                }
            }

            Stmt::While { condition, body } => {
                
                self.resolve_expr(condition);
                
                self.resolve_stmt(body);
            }


            Stmt::Return { keyword, value } => {
                if self.current_function == FunctionType::None {
                    
                    self.error(keyword, "Can't return from top-level code.");
                
                }

                if let Some(value) = value {
                    
                    if self.current_function == FunctionType::Initializer {
                        self.error(keyword, "Can't return a value from an initializer.");
                    }

                    self.resolve_expr(value);
                
                }
            }
        }
    }

    fn resolve_expr(&mut self, expr: &Expression) {
        match expr {
            
            Expression::Variable { id, name } => {
                let still_initializing = self
                    .scopes
                    .last()
                    .is_some_and(|scope| scope.get(&name.lexeme) == Some(&false));

                if still_initializing {
                    self.error(name, "Can't read local variable in its own initializer.");
                }

                self.resolve_local(*id, name);
            }

            Expression::Assign { id, name, value } => {
                self.resolve_expr(value);
                self.resolve_local(*id, name);
            }

            Expression::Binary { left, right, .. } | Expression::Logical { left, right, .. } => {
                self.resolve_expr(left);
                self.resolve_expr(right);
            }

            Expression::Call { callee, arguments, .. } => {
                
                self.resolve_expr(callee);
                
                for arg in arguments {
                
                    self.resolve_expr(arg);
                
                }
            }

            Expression::Grouping { expression } => self.resolve_expr(expression),

            Expression::Unary { right, .. } => self.resolve_expr(right),

            Expression::Literal { .. } => {}

            Expression::Get {object, ..} => self.resolve_expr(object),

            Expression::Set {object, value, ..} => {
                self.resolve_expr(value);
                self.resolve_expr(object);
            },

            Expression::This {id, keyword } => {
                if self.current_class == ClassType::None {
                    self.error(keyword, "Can't use 'this' outside of a class.");
                    return;
                }

                self.resolve_local(*id, keyword);
            }
        }
    }

    fn resolve_function(&mut self, decl: &FunctionDecl, kind: FunctionType) {
        let enclosing = self.current_function;
        self.current_function = kind;

       
       
        self.begin_scope();
       
        for param in &decl.params {
       
            self.declare(param);
            self.define(param);
       
        }
       
        self.resolve_stmts(&decl.body);
        self.end_scope();

       
        self.current_function = enclosing;
    }

    fn begin_scope(&mut self) {
        self.scopes.push(HashMap::new());
    }

    fn end_scope(&mut self) {
        self.scopes.pop();
    }

    fn declare(&mut self, name: &Token) {
        // at global scope there's nothing to track
        let already_declared = match self.scopes.last() {
       
            None => return,
       
            Some(scope) => scope.contains_key(&name.lexeme),
       
        };

        if already_declared {
       
            self.error(name, "Already a variable with this name in this scope.");
       
        }

        if let Some(scope) = self.scopes.last_mut() {
       
            scope.insert(name.lexeme.clone(), false);
        }
    }

    fn define(&mut self, name: &Token) {
       
        if let Some(scope) = self.scopes.last_mut() {
            scope.insert(name.lexeme.clone(), true);
        }
    }

    fn resolve_local(&mut self, id: usize, name: &Token) {
       
        for (depth, scope) in self.scopes.iter().rev().enumerate() {
            
            if scope.contains_key(&name.lexeme) {
            
                self.locals.insert(id, depth);
            
                return;
            }
        }
        //if its not in any scope, it's global, so we can leave it out of the map... 
    }

    fn error(&mut self, token: &Token, message: &str) {
        
        self.errors.push(ResolveError {
        
            token: token.clone(),
        
            message: message.to_string(),
        
        });
    }
}


#[cfg(test)]
mod tests {
    use super::*;
    use crate::parser::Parser;
    use crate::scanner::Scanner;

    fn parse(source: &str) -> Vec<Stmt> {
        let tokens = Scanner::new(source.to_string()).scan_tokens().clone();
        Parser::new(tokens).parse().expect("parse failed")
    }

    fn resolve_errors(source: &str) -> Vec<String> {
        match Resolver::new().resolve(&parse(source)) {
            Ok(_) => Vec::new(),
            Err(errors) => errors.into_iter().map(|e| e.message).collect(),
        }
    }

    fn resolve_depths(source: &str) -> Vec<usize> {
        let locals = Resolver::new().resolve(&parse(source)).expect("resolve failed");
        let mut depths: Vec<usize> = locals.into_values().collect();
        depths.sort();
        depths
    }

    #[test]
    fn reading_local_in_its_own_initializer_is_an_error() {
        assert_eq!(
            resolve_errors("{ var a = \"outer\"; { var a = a; } }"),
            vec!["Can't read local variable in its own initializer."]
        );
    }

    #[test]
    fn global_initializer_can_mention_itself() {
        assert!(resolve_errors("var a = a;").is_empty());
    }

    #[test]
    fn duplicate_local_is_an_error() {
        assert_eq!(
            resolve_errors("{ var a = 1; var a = 2; }"),
            vec!["Already a variable with this name in this scope."]
        );
    }

    #[test]
    fn duplicate_parameters_are_an_error() {
        assert_eq!(
            resolve_errors("fun f(a, a) {}"),
            vec!["Already a variable with this name in this scope."]
        );
    }

    #[test]
    fn globals_can_be_redeclared() {
        assert!(resolve_errors("var a = 1; var a = 2;").is_empty());
    }

    #[test]
    fn shadowing_in_an_inner_scope_is_fine() {
        assert!(resolve_errors("{ var a = 1; { var a = 2; } }").is_empty());
    }

    #[test]
    fn top_level_return_is_an_error() {
        assert_eq!(resolve_errors("return 1;"), vec!["Can't return from top-level code."]);
    }

    #[test]
    fn return_inside_a_function_is_fine() {
        assert!(resolve_errors("fun f() { return 1; }").is_empty());
    }

    #[test]
    fn reports_every_error_not_just_the_first() {
        assert_eq!(resolve_errors("return 1; { var a = 1; var a = 2; }").len(), 2);
    }

    #[test]
    fn globals_are_not_in_the_map() {
        assert!(resolve_depths("var a = 1; print a; a = 2;").is_empty());
    }

    #[test]
    fn local_in_same_scope_is_depth_zero() {
        assert_eq!(resolve_depths("{ var a = 1; print a; }"), vec![0]);
    }

    #[test]
    fn local_in_enclosing_scope_is_depth_one() {
        assert_eq!(resolve_depths("{ var a = 1; { print a; } }"), vec![1]);
    }

    #[test]
    fn parameters_resolve_inside_the_body() {
        assert_eq!(resolve_depths("fun f(x) { return x; }"), vec![0]);
    }

    #[test]
    fn assignment_gets_resolved_too() {
        assert_eq!(resolve_depths("{ var a; { a = 1; } }"), vec![1]);
    }

    #[test]
    fn error_message_format_matches_jlox() {
        let errors = Resolver::new().resolve(&parse("{ var a = 1;\nvar a = 2; }")).unwrap_err();
        assert_eq!(
            errors[0].to_string(),
            "[line 2] Error at 'a': Already a variable with this name in this scope."
        );
    }

        #[test]
    fn this_at_top_level_is_an_error() {
        assert_eq!(resolve_errors("print this;"), vec!["Can't use 'this' outside of a class."]);
    }

    #[test]
    fn this_in_a_plain_function_is_an_error() {
        assert_eq!(
            resolve_errors("fun notAMethod() { print this; }"),
            vec!["Can't use 'this' outside of a class."]
        );
    }

    #[test]
    fn this_inside_a_method_is_fine() {
        assert!(resolve_errors("class A { m() { return this; } }").is_empty());
    }

    #[test]
    fn this_in_a_function_nested_in_a_method_is_fine() {
        assert!(resolve_errors("class A { m() { fun f() { return this; } return f; } }").is_empty());
    }

    #[test]
    fn returning_a_value_from_init_is_an_error() {
        assert_eq!(
            resolve_errors("class Foo { init() { return \"something else\"; } }"),
            vec!["Can't return a value from an initializer."]
        );
    }

    #[test]
    fn bare_return_in_init_is_fine() {
        assert!(resolve_errors("class Foo { init() { return; } }").is_empty());
    }

    #[test]
    fn returning_a_value_from_a_normal_method_is_fine() {
        assert!(resolve_errors("class A { m() { return 1; } }").is_empty());
    }

    #[test]
    fn plain_function_named_init_can_return_a_value() {
        assert!(resolve_errors("fun init() { return 1; }").is_empty());
    }

    #[test]
    fn this_resolves_one_scope_above_the_method_body() {
  
        assert_eq!(resolve_depths("class A { m() { return this; } }"), vec![1]);
    }
}