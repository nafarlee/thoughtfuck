use std::io::{Read,Write,Result};
use std::fs::File;
use std::str;
use thoughtfuck::vm::*;
use thoughtfuck::program::*;
use thoughtfuck::parse::parse;


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
    {
        let mut vm = VM::new(&mut stdout);
        program.execute(&mut vm);
    }
    assert_eq!(stdout.buffer, read_file(expected_answer_filename));
}


struct FauxStdout {
    pub buffer: String
}


impl FauxStdout {
    pub fn new() -> FauxStdout {
        FauxStdout { buffer: String::new() }
    }
}


impl Write for FauxStdout {
    fn write(&mut self, buf: &[u8]) -> Result<usize> {
        match str::from_utf8(buf) {
            Err(e) => panic!(e),
            Ok(s) => {
                self.buffer += s;
                return Ok(s.len());
            }
        }
    }

    fn flush(&mut self) -> Result<()> {
        Ok(())
    }
}
