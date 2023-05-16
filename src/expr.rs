use crate::Token;
pub enum Expr {
    Binary(Box<Expr>, Token, Box<Expr>),
    Grouping(Box<Expr>),
    Literal(Token),
    Unary(Token, Box<Expr>),
}

impl Expr {
    pub fn print(&self) {
        match self {
            Expr::Binary(left, tok, right) => {
                print!("(");
                tok.print();
                print!(" ");
                left.print();
                print!(" ");
                right.print();
                print!(")");
            }
            Expr::Grouping(e) => {
                print!("(group");
                print!(" ");
                e.print();
                print!(")");
            }
            Expr::Literal(tok) => tok.print(),
            Expr::Unary(t, e) => {
                print!("(");
                t.print();
                print!(" ");
                e.print();
                print!(")");
            } //_ => {
              //println!("unsupported expr print")
              //}
        }
    }
}
