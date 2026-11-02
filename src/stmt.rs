use crate::expression::Expression;
use crate::token::Token;
use std::rc::Rc;

#[derive(Debug)]
pub struct FunctionDecl {
    pub name: Token, 
    pub params: Vec<Token>,
    pub body: Vec<Stmt>,
}


#[derive(Debug, Clone)]
pub enum Stmt {
    Expression(Expression),
    Print(Expression),
    Var {
        name: Token,
        initializer: Option<Expression> 
    },

    Block(Vec<Stmt>),

    If {
        condition: Expression,
        then_branch: Box<Stmt>,
        else_branch: Option<Box<Stmt>>,
    },

    While {
        condition: Expression,
        body: Box<Stmt>
    },

    Function(Rc<FunctionDecl>),

    Return {
        keyword: Token,
        value: Option<Expression>,
    }
}

