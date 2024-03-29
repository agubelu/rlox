mod debug;
mod parsing;
mod runtime;
mod scanning;
mod values;

use std::env;
use std::fs::read_to_string;
use std::process::exit;

use runtime::VM;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        println!("Usage: rlox <file.lox>");
        exit(1);
    } else {
        run_from_file(&args[1]);
    }
}

fn run_from_file(filename: &str) -> ! {
    let file_contents = read_to_string(filename)
                            .expect("Could not find/read the provided file.");
    let mut vm = VM::new();
    let result = vm.interpret(&file_contents);

    exit(result.exit_code());
}
