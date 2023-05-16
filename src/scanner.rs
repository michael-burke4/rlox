use crate::{LoxError, Token, TokenType};
use std::collections::HashMap;

#[derive(Debug)]
pub struct Scanner {
    source: String,
    tokens: Vec<Token>,
    current: usize,
    line: usize,
    keyword_map: HashMap<String, TokenType>,
}

// this wonderful macro has been provided by
// https://stackoverflow.com/a/28392068
// allows for simple hashmap initializations, like
// let x: HashMap<i64, char> = [10 => 'c', 20 => 'd', 30 => 'm'];
macro_rules! hashmap {
    ($( $key: expr => $val: expr ),*) => {{
         let mut map = ::std::collections::HashMap::new();
         $( map.insert($key, $val); )*
         map
    }}
}

impl Scanner {
    pub fn new(src: String) -> Scanner {
        // when adding new keywords, don't forget to change the keyword scanning in
        // scan_token() !!!
        let km: HashMap<String, TokenType> = hashmap![
            "and".to_string()    => TokenType::And,
            "class".to_string()  => TokenType::Class,
            "else".to_string()   => TokenType::Else,
            "false".to_string()  => TokenType::False,
            "fun".to_string()    => TokenType::Fun,
            "for".to_string()    => TokenType::For,
            "if".to_string()     => TokenType::If,
            "nil".to_string()    => TokenType::Nil,
            "or".to_string()     => TokenType::Or,
            "print".to_string()  => TokenType::Print,
            "return".to_string() => TokenType::Return,
            "super".to_string()  => TokenType::Super,
            "this".to_string()   => TokenType::This,
            "true".to_string()   => TokenType::True,
            "var".to_string()    => TokenType::Var,
            "while".to_string()  => TokenType::While
        ];
        Scanner {
            source: src,
            tokens: vec![],
            current: 0,
            line: 1,
            keyword_map: km,
        }
    }

    fn at_end(&self) -> bool {
        self.current >= self.source.len()
    }

    pub fn scan_tokens(&mut self) -> Result<(), LoxError> {
        while !self.at_end() {
            match self.scan_token() {
                Err(e) => return Err(e),
                _ => {
                    continue;
                }
            }
        }
        Ok(())
    }

    pub fn len(&self) -> usize {
        self.tokens.len()
    }

    pub fn token_at(&self, at: usize) -> &Token {
        &self.tokens[at]
    }

    fn scan_token(&mut self) -> Result<(), LoxError> {
        let a = self.advance();
        match a {
            ' ' => {}
            '\t' => {}
            '\r' => {}
            '\n' => {
                self.line += 1;
            }
            '(' => {
                self.add_token(Token::new(self.line, TokenType::LParen));
            }
            ')' => {
                self.add_token(Token::new(self.line, TokenType::RParen));
            }
            '{' => {
                self.add_token(Token::new(self.line, TokenType::LBrace));
            }
            '}' => {
                self.add_token(Token::new(self.line, TokenType::RBrace));
            }
            ',' => {
                self.add_token(Token::new(self.line, TokenType::Comma));
            }
            '.' => {
                self.add_token(Token::new(self.line, TokenType::Dot));
            }
            ';' => {
                self.add_token(Token::new(self.line, TokenType::Semicolon));
            }
            '-' => {
                self.add_token(Token::new(self.line, TokenType::Minus));
            }
            '+' => {
                self.add_token(Token::new(self.line, TokenType::Plus));
            }
            '*' => {
                self.add_token(Token::new(self.line, TokenType::Star));
            }
            '!' => {
                let matches = self.matches_expected('='); // CANNOT put directly in if condition: double &mut borrow :(
                self.add_token(if matches {
                    Token::new(self.line, TokenType::BangEqual)
                } else {
                    Token::new(self.line, TokenType::Bang)
                });
            }
            '=' => {
                let matches = self.matches_expected('=');
                self.add_token(if matches {
                    Token::new(self.line, TokenType::EqualEqual)
                } else {
                    Token::new(self.line, TokenType::Equal)
                });
            }
            '<' => {
                let matches = self.matches_expected('=');
                self.add_token(if matches {
                    Token::new(self.line, TokenType::LessEqual)
                } else {
                    Token::new(self.line, TokenType::Less)
                });
            }
            '>' => {
                let matches = self.matches_expected('=');
                self.add_token(if matches {
                    Token::new(self.line, TokenType::GreaterEqual)
                } else {
                    Token::new(self.line, TokenType::Greater)
                });
            }
            '/' => {
                if self.matches_expected('/') {
                    while !self.at_end() && self.curr_char() != '\n' {
                        self.advance();
                    }
                } else {
                    self.add_token(Token::new(self.line, TokenType::Slash));
                }
            }
            '"' => {
                let start = self.current;
                while !self.at_end() && self.curr_char() != '"' {
                    if self.curr_char() == '\n' {
                        self.line += 1;
                    }
                    self.advance();
                }
                if self.at_end() {
                    return Err(LoxError {
                        msg: "Unclosed string".to_string(),
                        line_no: self.line,
                    });
                }
                let s: String = std::str::from_utf8(&self.source.as_bytes()[start..self.current])
                    .unwrap()
                    .to_string();
                self.advance();
                println!("adding string with len {}", s.len());
                self.add_token(Token::new(self.line, TokenType::String(s)));
            }
            '0'..='9' => {
                let start = self.current;
                while self.curr_char().is_ascii_digit() {
                    self.advance();
                }
                if self.curr_char() == '.' && self.next_char().is_ascii_digit() {
                    self.advance();
                    while self.curr_char().is_ascii_digit() {
                        self.advance();
                    }
                }
                self.add_token(Token::new(
                    self.line,
                    TokenType::Number(self.source[start - 1..self.current].parse::<f64>().unwrap()),
                ));
            }
            'a'..='z' | 'A'..='Z' | '_' => {
                let start = self.current;
                while self.curr_char().is_ascii_alphanumeric() {
                    self.advance();
                }

                let lexeme = self.source[start - 1..self.current].to_string();
                match self.keyword_map.get(&lexeme) {
                    Some(t) => {
                        match t {
                              TokenType::And
                            | TokenType::Class
                            | TokenType::Else
                            | TokenType::False
                            | TokenType::Fun
                            | TokenType::For
                            | TokenType::If
                            | TokenType::Nil
                            | TokenType::Or
                            | TokenType::Print
                            | TokenType::Return
                            | TokenType::Super
                            | TokenType::This
                            | TokenType::True
                            | TokenType::Var
                            | TokenType::While => {
                                self.add_token(Token::new(self.line, t.clone()));
                            }
                            _ => panic!("GOT IMPOSSIBLE TOKEN TYPE FROM SCANNER KEYWORD MAP?!"),
                        }
                    }
                    None => {
                        self.add_token(Token::new(self.line, TokenType::Identifier(lexeme)));
                    }
                }
            }
            _ => {
                let err_msg = std::format!("Unrecognized character '{}'", a);
                return Err(LoxError {
                    msg: err_msg,
                    line_no: self.line,
                });
            }
        }
        Ok(())
    }

    fn advance(&mut self) -> char {
        let prev = self.curr_char();
        self.current += 1;
        prev
    }

    fn add_token(&mut self, t: Token) {
        self.tokens.push(t);
    }

    fn curr_char(&self) -> char {
        if self.at_end() {
            return '\0';
        }
        self.source.as_bytes()[self.current] as char
    }

    pub fn print_source(&self) {
        println!("{}", self.source);
    }

    pub fn print_tokens(&self) {
        for token in &self.tokens {
            println!("{:?}", token);
        }
    }

    fn matches_expected(&mut self, expected: char) -> bool {
        if self.at_end() {
            return false;
        }
        if self.curr_char() != expected {
            return false;
        }

        self.current += 1;
        true
    }

    fn next_char(&mut self) -> char {
        if self.current + 1 >= self.source.len() {
            return '\0';
        }
        self.source.as_bytes()[self.current + 1] as char
    }
}
