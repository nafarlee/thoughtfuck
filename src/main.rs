mod vm;
mod command;
mod parse;

use vm::VM;
use std::io::Write;

fn main() {
    let mut vm = VM::new();
    loop {
        print!("{}", "bf> ");
        std::io::stdout().flush().unwrap();

        let mut line = String::new();
        let commands = match std::io::stdin().read_line(&mut line) {
            Ok(0) => break,
            Ok(_) => parse::parse(&line),
            Err(error) => panic!(error),
        };
    }
}
