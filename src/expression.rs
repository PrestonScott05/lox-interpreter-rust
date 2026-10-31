use crate::token::Literal;
use crate::token::Token;



//i'm using Box<Expression> as a node so we can make a tree
#[derive(Debug, Clone, PartialEq)]
pub enum Expression {
    Literal { value: Literal },

    Grouping { expression: Box<Expression> },

    Unary { operator: Token, right: Box<Expression> },

    Binary { left: Box<Expression>, operator: Token, right: Box<Expression> }, 

    Variable { name: Token },

    Assign {
        name: Token,
        value: Box <Expression>
    },

    Logical {
        left: Box<Expression>,
        operator: Token,
        right: Box<Expression>
    },

}

//we can actually use match the correct way bc rust is great. this means we can skip almost all of chapter 5 (i found this out after doing it all...)
#[allow(dead_code)] // using this override because eour tests need it!
pub fn print(expr: &Expression) -> String {
    match expr {
        Expression::Literal {value} => {
            match value {  
                Literal::Number(n) => n.to_string(),
                Literal::Str(s) => s.clone(),
                Literal::Bool(b) => b.to_string(),
                Literal::Nil => "nil".to_string(),
            }
        },

        Expression::Grouping { expression } => format!("(group {})", print(expression)),
       
        Expression::Unary {operator , right } => format!("({} {})", operator.lexeme, print(right)),
    
        Expression::Binary {left, operator, right} => format!("({} {} {})",  operator.lexeme, print(left), print(right)),

        Expression::Variable {name } => name.lexeme.clone(),

        Expression::Assign { name, value } => format!("(= {} {})", name.lexeme, print(value)),

        Expression::Logical { left, operator, right } => format!("({} {} {})", operator.lexeme, print(left), print(right)),
    }
}


#[cfg(test)] 
mod tests {
    use super::*;
    use crate::token_type::TokenType;
    #[test]
    fn prints_nested_tree() {
        let expression = Expression::Binary {
            left: Box::new(Expression::Unary {
                operator: Token::new(TokenType::Minus, "-".to_string(), None, 1),
                right: Box::new(Expression::Literal {
                    value: Literal::Number(123.0),
                }),
            }),
            operator: Token::new(TokenType::Star, "*".to_string(), None, 1),
            right: Box::new(Expression::Grouping {
                expression: Box::new(Expression::Literal {
                    value: Literal::Number(45.67),
                }),
            }),
        };

        assert_eq!(print(&expression), "(* (- 123) (group 45.67))");
    }
}