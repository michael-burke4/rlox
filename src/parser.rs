use crate::{Expr, Scanner, Token, TokenType};
pub struct Parser {
    scanner: Scanner,
    current: usize,
}

// check if two instances of an enum share a variant.
// courtesy of
// https://stackoverflow.com/a/32554326
fn variant_eq<T>(a: &T, b: &T) -> bool {
    std::mem::discriminant(a) == std::mem::discriminant(b)
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

    fn consume(&mut self, tp: TokenType) -> bool {
        if !self.at_end() && variant_eq(&tp, &self.peek().typ) {
            self.advance();
            return true;
        }
        false
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
                self.advance();
            }
            TokenType::LParen => {
                self.advance();
                let e = self.expression();
                ret = Expr::Grouping(Box::new(e));
                if !self.consume(TokenType::RParen) {
                    println!("bad token on line {}", self.peek().line_no);
                }
            }
            _ => {
                panic!("bad error handling right now, but found bad primary token.");
            }
        }
        ret
    }

    fn synchronize(&mut self) {
        self.advance();

        while !self.at_end() {
            match self.previous().unwrap().typ {
                TokenType::Semicolon => return,
                _ => {}
            }
            match self.peek().typ {
                TokenType::Class
                | TokenType::Fun
                | TokenType::Var
                | TokenType::For
                | TokenType::If
                | TokenType::While
                | TokenType::Print
                | TokenType::Return => {
                    return;
                }
                _ => {}
            }
            self.advance();
        }
    }

    pub fn print_tokens(&self) {
        self.scanner.print_tokens();
    }
}
