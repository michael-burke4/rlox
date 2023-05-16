use crate::{Expr, Scanner, Token, TokenType};
pub struct Parser {
    scanner: Scanner,
    current: usize,
}

impl Parser {
    pub fn new(scn: Scanner) -> Parser {
        Parser {
            scanner: scn,
            current: 0,
        }
    }

    pub fn parse(&mut self) -> Expr {
        self.expression()
    }

    fn advance(&mut self) {
        self.current += 1;
    }

    fn previous(&self) -> Option<Token> {
        match self.current {
            0 => None,
            _ => Some(self.scanner.token_at(self.current - 1).clone()),
        }
    }

    fn peek(&self) -> &Token {
        self.scanner.token_at(self.current)
    }

    fn at_end(&self) -> bool {
        return self.current == self.scanner.len();
    }

    fn expression(&mut self) -> Expr {
        self.equality()
    }

    fn equality(&mut self) -> Expr {
        let mut ret = self.comparison();
        loop {
            if self.at_end() {
                break;
            }
            match self.peek().typ {
                TokenType::BangEqual | TokenType::EqualEqual => {
                    self.advance();
                    let operator = self.previous().unwrap();
                    let right = self.comparison();
                    ret = Expr::Binary(Box::new(ret), operator, Box::new(right));
                }
                _ => {
                    break;
                }
            }
        }
        ret
    }

    fn comparison(&mut self) -> Expr {
        let mut ret = self.term();
        loop {
            if self.at_end() {
                break;
            }
            match self.peek().typ {
                TokenType::Greater
                | TokenType::GreaterEqual
                | TokenType::Less
                | TokenType::LessEqual => {
                    self.advance();
                    let operator = self.previous().unwrap();
                    let right = self.term();
                    ret = Expr::Binary(Box::new(ret), operator, Box::new(right));
                }
                _ => {
                    break;
                }
            }
        }
        ret
    }

    fn term(&mut self) -> Expr {
        let mut ret = self.factor();
        loop {
            if self.at_end() {
                break;
            }
            match self.peek().typ {
                TokenType::Minus | TokenType::Plus => {
                    self.advance();
                    let operator = self.previous().unwrap();
                    let right = self.factor();
                    ret = Expr::Binary(Box::new(ret), operator, Box::new(right));
                }
                _ => {
                    break;
                }
            }
        }
        ret
    }

    fn factor(&mut self) -> Expr {
        let mut ret = self.unary();
        loop {
            if self.at_end() {
                break;
            }
            match self.peek().typ {
                TokenType::Slash | TokenType::Star => {
                    self.advance();
                    let operator = self.previous().unwrap();
                    let right = self.unary();
                    ret = Expr::Binary(Box::new(ret), operator, Box::new(right));
                }
                _ => {
                    break;
                }
            }
        }
        ret
    }

    fn unary(&mut self) -> Expr {
        match self.peek().typ {
            TokenType::Bang | TokenType::Minus => {
                self.advance();
                let operator = self.previous().unwrap();
                let right = self.unary();
                return Expr::Unary(operator, Box::new(right));
            }
            _ => self.primary(),
        }
    }

    fn primary(&mut self) -> Expr {
        let ret: Expr;
        match self.peek().typ {
              TokenType::False
            | TokenType::True
            | TokenType::Nil
            | TokenType::String(_)
            | TokenType::Number(_) => {
                ret = Expr::Literal(self.peek().clone());
            }
            TokenType::LParen => {
                self.advance();
                let e = self.expression();
                ret = Expr::Grouping(Box::new(e));
            }
            _ => {
                panic!("bad error handling right now, but found bad primary token.");
            }
        }
        self.advance();
        return ret;
    }

    pub fn print_tokens(&self) {
        self.scanner.print_tokens();
    }
}
