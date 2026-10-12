extern crate thoughtfuck;

use thoughtfuck::*;

use std::env;
use std::error::Error;
use std::fs::File;
use std::io::{Read, Write};
use std::path::Path;

use errors::ImbalancedBrackets;
use parse::parse;
use program::Program;
use vm::VM;

fn main() -> Result<(), Box<dyn Error>> {
    let mut stdout = std::io::stdout();
    let mut stdin = std::io::stdin();
    let vm = VM::new(&mut stdin, &mut stdout);
    let program = Program::new();
    match env::args().nth(1) {
        None => repl(vm, program),
        Some(arg) => interpreter(arg, vm, program),
    }
}

fn repl(mut vm: VM, mut program: Program) -> Result<(), Box<dyn Error>> {
    loop {
        match program.stack_depth() {
            0 => print!("tf> "),
            x => print!("tf+{}> ", x),
        }
        std::io::stdout().flush()?;

        let mut line = String::new();
        let read_count = std::io::stdin().read_line(&mut line)?;
        if 0 == read_count {
            break;
        }

        program.append(&parse(&line));
        program.execute(&mut vm);
    }
    match program.stack_depth() {
        0 => Ok(()),
        _ => Err(ImbalancedBrackets.into()),
    }
}

fn interpreter(arg: String, mut vm: VM, mut program: Program) -> Result<(), Box<dyn Error>> {
    let path = Path::new(&arg);
    let mut contents = String::new();
    File::open(&path)
        .unwrap()
        .read_to_string(&mut contents)
        .unwrap();

    program.append(&parse(&contents));
    program.assert_brackets_match()?;
    program.execute(&mut vm);
    Ok(())
}
