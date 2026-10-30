use crate::token::{Literal, Token};
use crate::token_type::TokenType;
use crate::expression::Expression;

#[derive(Debug)]
pub struct ParseError {
    pub token: Token,
    pub message: String,
}

pub struct Parser {
    tokens: Vec<Token>,
    current: usize,
}


impl Parser {
    pub fn new(in_tokens: Vec<Token>) -> Parser {
        Parser {
            tokens: in_tokens,
            current: 0,
        }
    }

    // entry point - the error was already reported in error(), so callers just need to know it failed
    pub fn parse(&mut self) -> Result<Expression, ParseError> {
        self.expression()
    }

    fn expression(&mut self) -> Result<Expression, ParseError> {
        self.equality()
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

    fn parse_source(source: &str) -> Result<Expression, ParseError> {
        let mut scanner = Scanner::new(source.to_string());
        let tokens = scanner.scan_tokens().clone();
        Parser::new(tokens).parse()
    }

    fn parse_to_string(source: &str) -> String {
        print(&parse_source(source).expect("should have parsed"))
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
        let err = parse_source("(1 + 2").unwrap_err();
        assert_eq!(err.message, "Expected ')' after expression.");
        assert_eq!(err.token.token_type, TokenType::Eof);
    }

    #[test]
    fn missing_operand_is_an_error() {
        let err = parse_source("1 +").unwrap_err();
        assert_eq!(err.message, "Expected expression.");
    }
}
