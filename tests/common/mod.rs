use std::io::{Write,Result};
use std::str;


pub struct FauxStdout {
    buffer: String
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
