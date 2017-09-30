extern crate thoughtfuck;

use thoughtfuck::vm::*;
use thoughtfuck::program::*;
use thoughtfuck::parse::parse;

#[test]
fn hello_world () {
    const SOURCE: &str = "++++++++[>++++[>++>+++>+++>+<<<<-]>+>+>->>+[<]<-]>>.>---.+++++++..+++.>>.<-.<.+++.------.--------.>>+.>++.";
    let commands = parse(SOURCE);

    let mut program = Program::new();
    program.append(&commands);

    let mut vm = VM::new(None);
    program.execute(&mut vm);
}
