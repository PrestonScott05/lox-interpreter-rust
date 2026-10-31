use crate::expression::Expression;
use crate::token::Token;

#[derive(Debug, Clone)]
pub enum Stmt {
    Expression(Expression),
    Print(Expression),
    Var {
        name: Token,
        initializer: Option<Expression> 
    },

    Block(Vec<Stmt>),
}

