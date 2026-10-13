use thoughtfuck::*;

use std::env;
use std::error::Error;
use std::fs::read_to_string;
use std::io::Write;

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
        Some(arg) => interpreter(&arg, vm, program),
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
        match std::io::stdin().read_line(&mut line) {
            Err(e) => return Err(e.into()),
            Ok(0) => break,
            Ok(_) => {}
        }

        program.append(&parse(&line));
        program.execute(&mut vm)?;
    }
    match program.stack_depth() {
        0 => Ok(()),
        _ => Err(ImbalancedBrackets.into()),
    }
}

fn interpreter(arg: &str, mut vm: VM, mut program: Program) -> Result<(), Box<dyn Error>> {
    let contents = read_to_string(arg)?;
    program.append(&parse(&contents));
    program.assert_brackets_match()?;
    program.execute(&mut vm)?;
    Ok(())
}
