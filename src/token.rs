#[derive(Debug, Clone)]
pub struct Token {
    pub line_no: usize,
    pub typ: TokenType,
}
#[derive(Debug, Clone)]
pub enum TokenType {
    LParen,
    RParen,
    LBrace,
    RBrace,
    Comma,
    Dot,
    Minus,
    Plus,
    Semicolon,
    Slash,
    Star,

    Bang,
    BangEqual,
    Equal,
    EqualEqual,
    Greater,
    GreaterEqual,
    Less,
    LessEqual,

    Identifier(String),
    String(String),
    Number(f64),

    And,
    Class,
    Else,
    False,
    Fun,
    For,
    If,
    Nil,
    Or,
    Print,
    Return,
    Super,
    This,
    True,
    Var,
    While,
}

impl Token {
    pub fn print(&self) {
        match &self.typ {
            TokenType::LParen => print!("("),
            TokenType::RParen => print!(")"),
            TokenType::LBrace => print!("{{"),
            TokenType::RBrace => print!("}}"),
            TokenType::Comma => print!(","),
            TokenType::Dot => print!("."),
            TokenType::Minus => print!("-"),
            TokenType::Plus => print!("+"),
            TokenType::Semicolon => print!(";"),
            TokenType::Slash => print!("/"),
            TokenType::Star => print!("*"),

            TokenType::Bang => print!("!"),
            TokenType::BangEqual => print!("!="),
            TokenType::Equal => print!("="),
            TokenType::EqualEqual => print!("=="),
            TokenType::Greater => print!(">"),
            TokenType::GreaterEqual => print!(">="),
            TokenType::Less => print!("<"),
            TokenType::LessEqual => print!("<="),

            TokenType::Identifier(s) => print!("{}", s),
            TokenType::String(s) => print!("\"{}\"", s),
            TokenType::Number(f) => print!("{}", f),

            TokenType::And => print!("and"),
            TokenType::Class => print!("class"),
            TokenType::Else => print!("else"),
            TokenType::False => print!("false"),
            TokenType::Fun => print!("fun"),
            TokenType::For => print!("for"),
            TokenType::If => print!("if"),
            TokenType::Nil => print!("nil"),
            TokenType::Or => print!("or"),
            TokenType::Print => print!("print"),
            TokenType::Return => print!("return"),
            TokenType::Super => print!("super"),
            TokenType::This => print!("this"),
            TokenType::True => print!("true"),
            TokenType::Var => print!("var"),
            TokenType::While => print!("while"),
        }
    }
    pub fn new(line: usize, token_type: TokenType) -> Token {
        Token {
            line_no: line,
            typ: token_type,
        }
    }
}
