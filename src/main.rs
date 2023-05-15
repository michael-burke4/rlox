mod token;
mod scanner;
mod lerror;

pub use token::Token;
pub use scanner::Scanner;
pub use lerror::LoxError;

use std::io;

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
        scanner.print_tokens();
    }
}
