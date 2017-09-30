extern crate thoughtfuck;
mod common;

use common::FauxStdout;
use thoughtfuck::vm::*;
use thoughtfuck::program::*;
use thoughtfuck::parse::parse;

#[test]
fn hello_world () {
    const SOURCE: &str = "++++++++[>++++[>++>+++>+++>+<<<<-]>+>+>->>+[<]<-]>>.>---.+++++++..+++.>>.<-.<.+++.------.--------.>>+.>++.";
    let commands = parse(SOURCE);

    let mut program = Program::new();
    program.append(&commands);

    let mut vm = VM::new(Some(Box::new(FauxStdout::new())));
    program.execute(&mut vm);
}
