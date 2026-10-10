extern crate thoughtfuck;

use thoughtfuck::*;

use std::env;
use std::fs::File;
use std::io::{Read, Write};
use std::path::Path;

use parse::parse;
use program::Program;
use program::ProgramStatus;
use vm::VM;

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
            Err(error) => panic!("{}", error),
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
