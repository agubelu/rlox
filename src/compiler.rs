use crate::{Scanner, TokenType};

pub struct Compiler {

}

impl Compiler {
    pub fn new() -> Self {
        Self { }
    }

    pub fn compile(&mut self, source: &str) {
        let mut line = 0;
        let mut scanner = Scanner::new(source);

        loop {
            let token = scanner.scan_next_token();
            if token.line != line {
                line = token.line;
                print!("{line:>4} ");
            } else {
                print!("   | ");
            }

            println!("{token:?}");

            if let TokenType::Eof = token.kind {
                break;
            }
        }
    }
}