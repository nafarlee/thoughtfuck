extern crate thoughtfuck;

mod common;

use std::fs::File;
use std::io::Read;

use common::FauxStdout;
use thoughtfuck::vm::*;
use thoughtfuck::program::*;
use thoughtfuck::parse::parse;

fn read_file(filename: &str) -> String {
    let mut file = File::open(filename).unwrap();
    let mut source = String::new();
    file.read_to_string(&mut source).unwrap();
    source
}


fn test_output(input_filename: &str, expected_answer_filename: &str) {
    let source = read_file(input_filename);

    let commands = parse(&source);

    let mut program = Program::new();
    program.append(&commands);

    let mut stdout = FauxStdout::new();
    {
        let mut vm = VM::new(&mut stdout);
        program.execute(&mut vm);
    }
    assert_eq!(stdout.buffer, read_file(expected_answer_filename));
}


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
