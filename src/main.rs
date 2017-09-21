mod vm;
mod command;
mod parse;
mod program;

use vm::VM;
use program::Program;
use std::io::Write;
use std::env;


fn main() {
    for argument in env::args() {
        println!("{}", argument);
    }

    let mut vm = VM::new();
    let mut program = Program::new();
    loop {
        print!("{}", if program.is_seeking {"... "} else {"bf> "});
        std::io::stdout().flush().unwrap();

        let mut line = String::new();
        let commands = match std::io::stdin().read_line(&mut line) {
            Ok(0) => break,
            Ok(_) => parse::parse(&line),
            Err(error) => panic!(error),
        };
        program.append(&commands);
        program.execute(&mut vm);
    }
}


fn repl (mut vm: VM, mut program: Program) -> () {
    loop {
        print!("{}", if program.is_seeking {"... "} else {"bf> "});
        std::io::stdout().flush().unwrap();

        let mut line = String::new();
        let commands = match std::io::stdin().read_line(&mut line) {
            Ok(0) => break,
            Ok(_) => parse::parse(&line),
            Err(error) => panic!(error),
        };
        program.append(&commands);
        program.execute(&mut vm);
    }
}
