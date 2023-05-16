use crate::LoxError;
use crate::Token;
use std::collections::HashMap;

#[derive(Debug)]
pub struct Scanner {
    source: String,
    tokens: Vec<Token>,
    current: usize,
    line: usize,
    keyword_map: HashMap<String, Token>,
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
        let km: HashMap<String, Token> = hashmap![
            "and".to_string() => Token::And(0),
            "class".to_string() => Token::Class(0),
            "else".to_string() => Token::Else(0),
            "false".to_string() => Token::False(0),
            "fun".to_string() => Token::Fun(0),
            "for".to_string() => Token::For(0),
            "if".to_string() => Token::If(0),
            "nil".to_string() => Token::Nil(0),
            "or".to_string() => Token::Or(0),
            "print".to_string() => Token::Print(0),
            "return".to_string() => Token::Return(0),
            "super".to_string() => Token::Super(0),
            "this".to_string() => Token::This(0),
            "true".to_string() => Token::True(0),
            "var".to_string() => Token::Var(0),
            "while".to_string() => Token::While(0)
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
                self.add_token(Token::LParen(self.line));
            }
            ')' => {
                self.add_token(Token::RParen(self.line));
            }
            '{' => {
                self.add_token(Token::LBrace(self.line));
            }
            '}' => {
                self.add_token(Token::RBrace(self.line));
            }
            ',' => {
                self.add_token(Token::Comma(self.line));
            }
            '.' => {
                self.add_token(Token::Dot(self.line));
            }
            ';' => {
                self.add_token(Token::Semicolon(self.line));
            }
            '-' => {
                self.add_token(Token::Minus(self.line));
            }
            '+' => {
                self.add_token(Token::Plus(self.line));
            }
            '*' => {
                self.add_token(Token::Star(self.line));
            }
            '!' => {
                let matches = self.matches_expected('='); // CANNOT put directly in if condition: double &mut borrow :(
                self.add_token(if matches {
                    Token::BangEqual(self.line)
                } else {
                    Token::Bang(self.line)
                });
            }
            '=' => {
                let matches = self.matches_expected('=');
                self.add_token(if matches {
                    Token::EqualEqual(self.line)
                } else {
                    Token::Equal(self.line)
                });
            }
            '<' => {
                let matches = self.matches_expected('=');
                self.add_token(if matches {
                    Token::LessEqual(self.line)
                } else {
                    Token::Less(self.line)
                });
            }
            '>' => {
                let matches = self.matches_expected('=');
                self.add_token(if matches {
                    Token::GreaterEqual(self.line)
                } else {
                    Token::Greater(self.line)
                });
            }
            '/' => {
                if self.matches_expected('/') {
                    while !self.at_end() && self.curr_char() != '\n' {
                        self.advance();
                    }
                } else {
                    self.add_token(Token::Slash(self.line));
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
                self.add_token(Token::String(self.line, s));
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
                self.add_token(Token::Number(
                    self.line,
                    self.source[start - 1..self.current].parse::<f64>().unwrap(),
                ))
            }
            'a'..='z' | 'A'..='Z' | '_' => {
                let start = self.current;
                while self.curr_char().is_ascii_alphanumeric() {
                    self.advance();
                }

                let lexeme = self.source[start - 1..self.current].to_string();
                match self.keyword_map.get(&lexeme) {
                    Some(t) => {
                        let mut ret = t.clone();
                        // this is dumb and brittle, but I don't think there's an easier way
                        // given the way I've set this up :(
                        match &mut ret {
                            Token::And(val)
                            | Token::Class(val)
                            | Token::Else(val)
                            | Token::False(val)
                            | Token::Fun(val)
                            | Token::For(val)
                            | Token::If(val)
                            | Token::Nil(val)
                            | Token::Or(val)
                            | Token::Print(val)
                            | Token::Return(val)
                            | Token::Super(val)
                            | Token::This(val)
                            | Token::True(val)
                            | Token::Var(val)
                            | Token::While(val) => {
                                *val = self.line;
                            }
                            _ => panic!("GOT IMPOSSIBLE TOKEN TYPE FROM SCANNER KEYWORD MAP?!"),
                        }
                        self.add_token(ret);
                    }
                    None => {
                        self.add_token(Token::Identifier(self.line, lexeme));
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
