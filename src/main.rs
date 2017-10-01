extern crate thoughtfuck;

use thoughtfuck::*;

use std::io::{Write, Read};
use std::env;
use std::path::Path;
use std::fs::File;

use vm::VM;
use program::Program;
use program::ProgramStatus;
use parse::parse;


fn main() {
    let mut stdout = std::io::stdout();
    let vm = VM::new(&mut stdout);
    let program = Program::new();
    match env::args().nth(1) {
        None => repl(vm, program),
        Some(arg) => interpreter(arg, vm, program),
    }
}


fn repl(mut vm: VM, mut program: Program) -> () {
    loop {
        if let ProgramStatus::Normal = program.status {
            print!("tf> ");
        } else {
            print!("... ");
        }
        std::io::stdout().flush().unwrap();

        let mut line = String::new();
        let commands = match std::io::stdin().read_line(&mut line) {
            Ok(0) => break,
            Ok(_) => parse(&line),
            Err(error) => panic!(error),
        };
        program.append(&commands);
        program.execute(&mut vm);
    }
}


fn interpreter(arg: String, mut vm: VM, mut program: Program) -> () {
    let path = Path::new(&arg);
    let mut contents = String::new();
    File::open(&path)
        .unwrap()
        .read_to_string(&mut contents)
        .unwrap();

    program.append(&parse(&contents));
    program.execute(&mut vm);
}
