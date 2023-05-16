mod expr;
mod lerror;
mod parser;
mod scanner;
mod token;

pub use expr::Expr;
pub use lerror::LoxError;
pub use parser::Parser;
pub use scanner::Scanner;
pub use token::Token;
pub use token::TokenType;

fn main() {
    let args = std::env::args();
    if args.count() != 2 {
        eprintln!("Usage: `rlox <filename.lox>`");
        std::process::exit(1);
    }
    for arg in std::env::args().skip(1) {
        let file_contents = std::fs::read_to_string(arg).unwrap();
        let mut scanner = Scanner::new(file_contents);

        match scanner.scan_tokens() {
            Err(e) => eprintln!("[Line {}] {}", e.line_no, e.msg),
            _ => (),
        }
        let mut parser = Parser::new(scanner);
        //parser.print_tokens();
        parser.parse().print();
    }

    println!();
}
