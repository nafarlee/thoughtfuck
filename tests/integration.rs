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

    let mut stdout = FauxStdout::new();
    {
        let mut vm = VM::new(&mut stdout);
        program.execute(&mut vm);
    }
    assert_eq!(stdout.buffer, "Hello World!\n");
}
