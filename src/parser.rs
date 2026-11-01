use crate::token::{Literal, Token};
use crate::token_type::TokenType;
use crate::expression::Expression;
use crate::stmt::{Stmt};

#[derive(Debug)]
pub struct ParseError {
    pub token: Token,
    pub message: String,
}

pub struct Parser {
    tokens: Vec<Token>,
    current: usize,
    errors: Vec<ParseError>,
}


impl Parser {
    pub fn new(in_tokens: Vec<Token>) -> Parser {
        Parser {
            tokens: in_tokens,
            current: 0,
            errors: Vec::new(),
        }
    }

   
    pub fn parse(&mut self) -> Result<Vec<Stmt>, Vec<ParseError>> {
        let mut statements = Vec::new();
        while !self.is_at_end() {
            if let Some(stmt) = self.declaration() {
                statements.push(stmt);
            }
        }
        if self.errors.is_empty() {
            Ok(statements)
        }
        else {
            Err(std::mem::take(&mut self.errors))
        }
        
    }

    fn declaration(&mut self) -> Option<Stmt> {
        let result = if self.match_token(&[TokenType::Var]) {
            self.var_declaration()
        } else {
            self.statement()
        };

        match result {
            Ok(stmt) => Some(stmt),

            Err(e) => {
                self.errors.push(e);
                self.synchronize();
                None
            }
        }
    }

    fn var_declaration(&mut self) -> Result<Stmt, ParseError> {
        let name = self.consume(TokenType::Identifier, "Expect variable name.")?;

        let initializer = if self.match_token(&[TokenType::Equal]) {
            Some(self.expression()?)
        } else {
            None
        };

        self.consume(TokenType::Semicolon, "Expect ';' after variable declaration.")?;
        Ok(Stmt::Var { name, initializer })
    }

    fn statement(&mut self) -> Result<Stmt, ParseError> {
        if self.match_token(&[TokenType::For]) {
            return self.for_statement()
        }
        if self.match_token(&[TokenType::If]) {
            return self.if_statement()
        }
        if self.match_token(&[TokenType::While]) {
            return self.while_statement()
        }

        if self.match_token(&[TokenType::Print]) {
            return self.print_statement()
        }
        
        if self.match_token(&[TokenType::LeftBrace]) {
            return Ok(Stmt::Block(self.block()?));
        }
        self.expression_statement()
    }

    fn block(&mut self) -> Result<Vec<Stmt>, ParseError> {
        let mut statements = Vec::new();

        while !self.check(TokenType::RightBrace) && !self.is_at_end() {
            if let Some(stmt) = self.declaration() {
                statements.push(stmt);
            }
        }

        self.consume(TokenType::RightBrace, "Expect '}' after block.")?;
        
        Ok(statements)
    }

    fn if_statement(&mut self) -> Result<Stmt, ParseError> {
        self.consume(TokenType::LeftParen, "Expect '(' after 'if'.")?;

        let condition = self.expression()?;

        self.consume(TokenType::RightParen, "Expect ')' after if condition.")?;

        let then_branch = self.statement()?;
        let else_branch = if self.match_token(&[TokenType::Else]) {
            Some(Box::new(self.statement()?))
        } else {
            None
        };

        Ok(Stmt::If {condition, then_branch: Box::new(then_branch), else_branch})
    }

    fn while_statement(&mut self) -> Result<Stmt, ParseError> {
        self.consume(TokenType::LeftParen, "Expect '(' after 'while'.")?;

        let condition = self.expression()?;

        self.consume(TokenType::RightParen, "Expect ')' after condition.")?; 

        let body = self.statement()?;

        Ok(Stmt::While {
            condition, 
            body: Box::new(body),
        })
    }

    fn for_statement(&mut self) -> Result<Stmt, ParseError> {
        self.consume(TokenType::LeftParen, "Expect '(' after 'for'.")?;

        let initializer = if self.match_token(&[TokenType::Semicolon]) {
            None
        } else if self.match_token(&[TokenType::Var]) {
            Some(self.var_declaration()?)
        } else {
            Some(self.expression_statement()?)
        };

        let condition = if !self.check(TokenType::Semicolon) {
            Some(self.expression()?)
        } else {
            None
        };
        self.consume(TokenType::Semicolon, "Expect ';' after loop condition.")?;

        let increment = if !self.check(TokenType::RightParen) {
            Some(self.expression()?)
        } else {
            None
        };
        self.consume(TokenType::RightParen, "Expect ')' after for clauses.")?;

        let mut body = self.statement()?;


        if let Some(inc) = increment {
            body = Stmt::Block(vec![body, Stmt::Expression(inc)]);
        }

  
        let condition = condition.unwrap_or(Expression::Literal { value: Literal::Bool(true) });
        body = Stmt::While {

            condition,

            body: Box::new(body),
        };

        
        if let Some(init) = initializer {
            body = Stmt::Block(vec![init, body]);
        }

        Ok(body)
    }

    fn print_statement(&mut self) -> Result<Stmt, ParseError> {
        let val = self.expression()?;

        self.consume(TokenType::Semicolon, "Expected a ';' after the value.")?;

        Ok(Stmt::Print(val))
    }

    fn expression_statement(&mut self) -> Result<Stmt, ParseError> {
        let expr = self.expression()?;

        self.consume(TokenType::Semicolon, "Expected a ';' after the expression")?;

        Ok(Stmt::Expression(expr))
    }

    fn expression(&mut self) -> Result<Expression, ParseError> {
        self.assignment()
    }

    fn assignment(&mut self) -> Result<Expression, ParseError> {
         let expr = self.or()?;

        if self.match_token(&[TokenType::Equal]) {
            let equals = self.previous();
            let value = self.assignment()?; //recurse!

            return match expr {
                Expression::Variable { name } => Ok(Expression::Assign {
                    name,
                    value: Box::new(value),
                }),

                other => {
                    let err = self.error(equals, "Invalid assignment target.");
                    self.errors.push(err); //we should repor, but not return the err
                    Ok(other)
                }
            };
        }

        Ok(expr)
    }

    fn error(&self, token:Token, message: &str) -> ParseError {
        let location = if token.token_type == TokenType::Eof {
            " at end".to_string()
        } else {
            format!(" at '{}'", token.lexeme)
        };

        eprintln!("[line {}] Error{}: {}", token.line, location, message);

        ParseError {
            token,
            message: message.to_string(),
        }
    }

    // skips tokens until we're probably at the start of the next statement. not called until chapter 8 adds statements
    #[allow(dead_code)]
    fn synchronize(&mut self) {
        self.advance();

        while !self.is_at_end() {
            if self.previous().token_type == TokenType::Semicolon {
                return;
            }

            match self.peek().token_type {
                TokenType::Class | TokenType::Fun | TokenType::Var | TokenType::For
                | TokenType::If | TokenType::While | TokenType::Print | TokenType::Return => return,
                _ => {}
            }

            self.advance();
        }
    }

    fn match_token(&mut self, types: &[TokenType]) -> bool {
        for ty in types {
            if self.check(*ty) {
                self.advance();
                return true;
            }
        }

        false
    }

    fn consume(&mut self, ty: TokenType, message: &str) -> Result<Token, ParseError> {
        if self.check(ty) {
            return Ok(self.advance());
        }
        Err(self.error(self.peek(), message))
    }

    fn check(& self, ty: TokenType) -> bool {
        if self.is_at_end() {
            return false;
        }
        self.peek().token_type == ty
    }

    fn is_at_end(&self) -> bool {
        self.peek().token_type == TokenType::Eof
    }

    fn peek(&self) -> Token {
        self.tokens[self.current].clone()
    }

    fn previous(&self) -> Token {
        self.tokens[self.current - 1].clone()
    }

    fn advance(&mut self) -> Token {
        if !self.is_at_end() {
            self.current += 1;
        }

        self.previous()
    }

    fn equality(&mut self) -> Result<Expression, ParseError> {
        let mut expr: Expression = self.comparison()?;

        while self.match_token(&[TokenType::BangEqual, TokenType::EqualEqual]) {
            let operator = self.previous();
            let right = self.comparison()?;
            expr = Expression::Binary {
                left: Box::new(expr),
                operator,
                right: Box::new(right)
            };
        }
        Ok(expr)
    }

    fn or(&mut self) -> Result<Expression, ParseError> {
        let mut expr: Expression = self.and()?;

        while self.match_token(&[TokenType::Or]) {
            let operator: Token = self.previous();

            let right: Expression = self.and()?;
            
            expr = Expression::Logical {
                left: Box::new(expr),
                operator,
                right: Box::new(right)
            };

        }
        Ok(expr)
    }

    fn and(&mut self) -> Result<Expression, ParseError> {
        let mut expr: Expression = self.equality()?;

        while self.match_token(&[TokenType::And]) {
            let operator = self.previous();
            let right = self.equality()?;

            expr = Expression::Logical {
                left: Box::new(expr),
                operator,
                right: Box::new(right),
            };
        }
        Ok(expr)
    }

    fn comparison(&mut self) -> Result<Expression, ParseError> {
        let mut expr: Expression = self.term()?;
        while self.match_token(&[TokenType::Greater, TokenType::GreaterEqual, TokenType::Less, TokenType::LessEqual]) {
            let operator: Token = self.previous();
            let right: Expression = self.term()?;
            expr = Expression::Binary {
                left: Box::new(expr),
                operator,
                right: Box::new(right)
            };
        }

        Ok(expr)
    }

    fn term(&mut self) -> Result<Expression, ParseError> {
        let mut expr: Expression = self.factor()?;

        while self.match_token(&[TokenType::Minus, TokenType::Plus]) {
            let operator: Token = self.previous();
            let right: Expression = self.factor()?;

            expr = Expression::Binary {
                left: Box::new(expr),
                operator,
                right: Box::new(right)
            };
        }
        Ok(expr)
    }

    fn factor(&mut self) -> Result<Expression, ParseError> {
        let mut expr: Expression = self.unary()?;

        while self.match_token(&[TokenType::Slash, TokenType::Star]) {
            let operator: Token = self.previous();
            let right: Expression = self.unary()?;

            expr = Expression::Binary {
                left: Box::new(expr),
                operator,
                right: Box::new(right)
            };
        }
        Ok(expr)
    }

    fn unary(&mut self) -> Result<Expression, ParseError> {
        if self.match_token(&[TokenType::Bang, TokenType::Minus]) {
            let operator: Token = self.previous();
            let right: Expression = self.unary()?;

            return Ok(Expression::Unary {
                operator,
                right: Box::new(right)
            });
        }
        self.primary()
    }

    fn primary(&mut self) -> Result<Expression, ParseError> {
        if self.match_token(&[TokenType::False]) {
            return Ok(Expression::Literal { value: Literal::Bool(false) });
        }

        if self.match_token(&[TokenType::True]) {
            return Ok(Expression::Literal {
                value: Literal::Bool(true)
            });
        }
        if self.match_token(&[TokenType::Nil]) {
            return Ok(Expression::Literal {
                value: Literal::Nil
            });
        }

        if self.match_token(&[TokenType::Number, TokenType::String]) {
            let value = self.previous().literal.expect("scanner bug: Number/String token with no literal");
            return Ok(Expression::Literal {
                value: value
            });
        }

        if self.match_token(&[TokenType::Identifier]) {
            return Ok(Expression::Variable {
                name: self.previous()
            });
        }

        if self.match_token(&[TokenType::LeftParen]) {
            let expr: Expression = self.expression()?;
            self.consume(TokenType::RightParen, "Expected ')' after expression.")?;

            return Ok(Expression::Grouping {
                expression: Box::new(expr)
            });
        }
        Err(self.error(self.peek(), "Expected expression."))
    }
}


#[cfg(test)]
mod tests {
    use super::*;
    use crate::expression::print;
    use crate::scanner::Scanner;
    use crate::stmt::Stmt;

    fn parse_source(source: &str) -> Result<Expression, Vec<ParseError>> {
        let mut scanner = Scanner::new(format!("{source};"));

        let tokens = scanner.scan_tokens().clone();

        let stmts = Parser::new(tokens).parse()?;

        match stmts.as_slice() {
            [Stmt::Expression(expression)] => Ok(expression.clone()),

            other => panic!("expected one expression statement, got {other:?}"),
        }
    }

    fn parse_to_string(source: &str) -> String {
        print(&parse_source(source).expect("should have parsed"))
    }

    fn parse_program(source: &str) -> Result<Vec<Stmt>, Vec<ParseError>> {
        let tokens = Scanner::new(source.to_string()).scan_tokens().clone();

        Parser::new(tokens).parse()
    }

    #[test]
    fn parses_literals() {
        assert_eq!(parse_to_string("123"), "123");
        assert_eq!(parse_to_string("\"hi\""), "hi");
        assert_eq!(parse_to_string("true"), "true");
        assert_eq!(parse_to_string("false"), "false");
        assert_eq!(parse_to_string("nil"), "nil");
    }

    #[test]
    fn factor_binds_tighter_than_term() {
        assert_eq!(parse_to_string("1 + 2 * 3"), "(+ 1 (* 2 3))");
    }

    #[test]
    fn binary_operators_are_left_associative() {
        assert_eq!(parse_to_string("1 - 2 - 3"), "(- (- 1 2) 3)");
    }

    #[test]
    fn parses_unary_and_grouping() {
        assert_eq!(parse_to_string("-(1 + 2)"), "(- (group (+ 1 2)))");
        assert_eq!(parse_to_string("!!true"), "(! (! true))");
    }

    #[test]
    fn full_precedence_chain() {
        assert_eq!(
            parse_to_string("1 + 2 < 3 == !false"),
            "(== (< (+ 1 2) 3) (! false))"
        );
    }

    #[test]
    fn missing_right_paren_is_an_error() {
        let errs = parse_source("(1 + 2").unwrap_err();
        assert_eq!(errs[0].message, "Expected ')' after expression.");
        assert_eq!(errs[0].token.token_type, TokenType::Semicolon);
    }

    #[test]
    fn missing_operand_is_an_error() {
        let errs = parse_source("1 +").unwrap_err();
        assert_eq!(errs[0].message, "Expected expression.");
    }

     #[test]
    fn parses_print_statement() {
        let tokens = Scanner::new("print 1 + 2;".to_string()).scan_tokens().clone();
        let stmts = Parser::new(tokens).parse().unwrap();
        assert!(matches!(stmts.as_slice(), [Stmt::Print(_)]));
    }

    #[test]
    fn parses_var_with_initializer() {
        let stmts = parse_program("var a = 1;").unwrap();

        assert!(matches!(
            stmts.as_slice(), 
            [Stmt::Var { initializer: Some(_), .. }]
        ));
    }

    #[test]
    fn parses_var_without_initializer() {
        let stmts = parse_program("var a;").unwrap();

        assert!(matches!(
            stmts.as_slice(), 
            [Stmt::Var { initializer: None, .. }]
        ));
    }

    #[test]
    fn parses_variable_expression() {
        assert_eq!(parse_to_string("a"), "a");
    }

    #[test]
    fn var_missing_name_is_error() {
        let errs = parse_program("var = 1;").unwrap_err();

        assert_eq!(errs[0].message, "Expect variable name.");
    }

    #[test]
    fn recovers_after_error_and_keepsparsing() {
        let errs = parse_program("var = 1; var = 2; print 3;").unwrap_err();

        assert_eq!(errs.len(), 2);
    }

    #[test]
    fn parses_assignment() {
        assert_eq!(parse_to_string("a = 1"), "(= a 1)");
    }

    #[test]
    fn assignment_is_right_associative() {
        assert_eq!(parse_to_string("a = b = c"), "(= a (= b c))");
    }

    #[test]
    fn invalid_assignment_targets() {
        assert_eq!(parse_source("a + b = c").unwrap_err()[0].message, "Invalid assignment target.");
        assert_eq!(parse_source("(a) = 3").unwrap_err()[0].message, "Invalid assignment target.");
    }

    #[test]
    fn parses_block() {
        let stmts = parse_program("{ var a = 1; print a; }").unwrap();
        match stmts.as_slice() {
            [Stmt::Block(inner)] => assert_eq!(inner.len(), 2),
            other => panic!("expected one block, got {other:?}"),
        }
    }

    #[test]
    fn unclosed_block_is_an_error() {
        let errs = parse_program("{ var a = 1;").unwrap_err();
        assert_eq!(errs[0].message, "Expect '}' after block.");
    }

    #[test]
    fn logical_precedence() {
        assert_eq!(parse_to_string("a or b and c"), "(or a (and b c))");
        assert_eq!(parse_to_string("a == b or c"), "(or (== a b) c)");
        assert_eq!(parse_to_string("a or b or c"), "(or (or a b) c)");
    }

    #[test]
    fn assignment_still_works_over_logical() {
        assert_eq!(parse_to_string("a = b or c"), "(= a (or b c))");
    }

    #[test]
    fn parses_if_else() {
        let stmts = parse_program("if (true) print 1; else print 2;").unwrap();
        assert!(matches!(stmts.as_slice(), [Stmt::If { else_branch: Some(_), .. }]));
    }

    #[test]
    fn dangling_else_binds_to_nearest_if() {
        let stmts = parse_program("if (a) if (b) print 1; else print 2;").unwrap();
        match stmts.as_slice() {
            [Stmt::If { then_branch, else_branch: None, .. }] => {
                assert!(matches!(**then_branch, Stmt::If { else_branch: Some(_), .. }));
            }
            other => panic!("else attached to the wrong if: {other:?}"),
        }
    }

    #[test]
    fn parses_while() {
        let stmts = parse_program("while (true) print 1;").unwrap();
        assert!(matches!(stmts.as_slice(), [Stmt::While { .. }]));
    }

    #[test]
    fn for_desugars_to_block_with_while() {
        let stmts = parse_program("for (var i = 0; i < 3; i = i + 1) print i;").unwrap();
        match stmts.as_slice() {
            [Stmt::Block(inner)] => {
                assert!(matches!(inner.as_slice(), [Stmt::Var { .. }, Stmt::While { .. }]));
            }
            other => panic!("expected desugared for, got {other:?}"),
        }
    }

    #[test]
    fn empty_for_is_just_a_while_true() {
        let stmts = parse_program("for (;;) print 1;").unwrap();
        assert!(matches!(
            stmts.as_slice(),
            [Stmt::While { condition: Expression::Literal { value: Literal::Bool(true) }, .. }]
        ));
    }

    #[test]
    fn missing_paren_after_if_is_an_error() {
        let errs = parse_program("if true) print 1;").unwrap_err();
        assert_eq!(errs[0].message, "Expect '(' after 'if'.");
    }
}
