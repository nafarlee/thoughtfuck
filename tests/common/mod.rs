mod faux_stdout;

use self::faux_stdout::FauxStdout;
use std::{fs::File, io::Read};
use thoughtfuck::{parse::parse, program::Program, vm::VM};

fn read_file(filename: &str) -> String {
    let mut file = File::open(filename).unwrap();
    let mut source = String::new();
    file.read_to_string(&mut source).unwrap();
    source
}

pub fn test_output(input_filename: &str, expected_answer_filename: &str) {
    let source = read_file(input_filename);

    let commands = parse(&source);

    let mut program = Program::new();
    program.append(&commands);

    let mut stdout = FauxStdout::new();
    let mut input = "".as_bytes();
    {
        let mut vm = VM::new(&mut input, &mut stdout);
        program.execute(&mut vm).unwrap();
    }
    assert_eq!(stdout.buffer, read_file(expected_answer_filename));
}
