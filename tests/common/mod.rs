use std::io::{Write,Result};
use std::str;


pub struct FauxStdout {
    buffer: String
}


impl Write for FauxStdout {
    fn write(&mut self, buf: &[u8]) -> Result<usize> {
        self.buffer += str::from_utf8(buf).expect("u8 slice provided to FauxStdout.write() was not valid UTF-8");
        Ok(0)
    }

    fn flush(&mut self) -> Result<()> {
        Ok(())
    }
}
