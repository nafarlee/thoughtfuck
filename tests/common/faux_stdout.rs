use std::{
    io::{self, Write},
    str,
};

pub struct FauxStdout {
    pub buffer: String,
}

impl FauxStdout {
    pub fn new() -> FauxStdout {
        FauxStdout {
            buffer: String::new(),
        }
    }
}

impl Write for FauxStdout {
    fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
        match str::from_utf8(buf) {
            Err(e) => panic!("{}", e),
            Ok(s) => {
                self.buffer += s;
                Ok(s.len())
            }
        }
    }

    fn flush(&mut self) -> io::Result<()> {
        Ok(())
    }
}
