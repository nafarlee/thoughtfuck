use crate::command::Command;
use crate::errors;
use std::error::Error;
use std::io::{Read, Write};

const TAPE_SIZE: usize = 30_000;
pub type Cell = u8;

pub struct VM<'a> {
    cells: [Cell; TAPE_SIZE],
    data_pointer: usize,
    out: &'a mut dyn Write,
    input: &'a mut dyn Read,
}

impl<'a> VM<'a> {
    pub fn new(input: &'a mut dyn Read, out: &'a mut dyn Write) -> VM<'a> {
        VM {
            cells: [0; TAPE_SIZE],
            data_pointer: 0,
            out,
            input,
        }
    }

    pub fn get(&self) -> Cell {
        self.cells[self.data_pointer]
    }

    pub fn output(&mut self) -> Result<(), Box<dyn Error>> {
        self.out.write_all(&[self.get()])?;
        Ok(())
    }

    pub fn input(&mut self) -> Result<(), Box<dyn Error>> {
        match self.input.bytes().next() {
            Some(Ok(value)) => self.cells[self.data_pointer] = value,
            Some(Err(e)) => return Err(e.into()),
            None => {}
        }
        Ok(())
    }

    pub fn increment(&mut self) {
        self.cells[self.data_pointer] = self.get().wrapping_add(1);
    }

    pub fn decrement(&mut self) {
        self.cells[self.data_pointer] = self.get().wrapping_sub(1);
    }

    pub fn right_shift(&mut self) -> Result<(), Box<dyn Error>> {
        if self.data_pointer < TAPE_SIZE - 1 {
            self.data_pointer += 1;
            Ok(())
        } else {
            Err(errors::TapeBoundsOverflow.into())
        }
    }

    pub fn left_shift(&mut self) -> Result<(), Box<dyn Error>> {
        let Some(x) = self.data_pointer.checked_sub(1) else {
            return Err(errors::TapeBoundsUnderflow.into());
        };
        self.data_pointer = x;
        Ok(())
    }

    pub fn apply(&mut self, command: Command) -> Result<(), Box<dyn Error>> {
        match command {
            Command::Output => self.output(),
            Command::Input => self.input(),
            Command::Increment => {
                self.increment();
                Ok(())
            }
            Command::Decrement => {
                self.decrement();
                Ok(())
            }
            Command::RightShift => self.right_shift(),
            Command::LeftShift => self.left_shift(),
            _ => Ok(()),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io;
    #[test]
    fn test_new() {
        let mut stdout = io::stdout();
        let mut input = "".as_bytes();
        let vm = VM::new(&mut input, &mut stdout);
        assert!(vm.cells[0] == 0);
        assert!(vm.cells[TAPE_SIZE - 1] == 0);
        assert!(vm.cells[0] == 0);
    }

    #[test]
    fn test_increment() {
        let mut stdout = io::stdout();
        let mut input = "".as_bytes();
        let mut vm = VM::new(&mut input, &mut stdout);
        vm.increment();
        assert!(vm.cells[0] == 1);
    }

    #[test]
    fn test_decrement() {
        const STARTING_VALUE: u8 = 2;

        let mut stdout = io::stdout();
        let mut input = "".as_bytes();
        let mut vm = VM::new(&mut input, &mut stdout);
        vm.cells[0] = STARTING_VALUE;
        vm.decrement();
        assert!(vm.cells[0] == STARTING_VALUE - 1);
    }

    #[test]
    fn test_right_shift() {
        let mut stdout = io::stdout();
        let mut input = "".as_bytes();
        let mut vm = VM::new(&mut input, &mut stdout);
        vm.right_shift().unwrap();
        assert!(vm.data_pointer == 1);
    }

    #[test]
    fn test_left_shift() {
        const STARTING_POSITION: usize = 2;

        let mut stdout = io::stdout();
        let mut input = "".as_bytes();
        let mut vm = VM::new(&mut input, &mut stdout);
        vm.data_pointer = STARTING_POSITION;
        vm.left_shift().unwrap();
        assert!(vm.data_pointer == 1);
    }

    #[test]
    fn test_output() {
        let mut out = vec![];
        let mut input = "".as_bytes();
        let mut vm = VM::new(&mut input, &mut out);
        vm.cells[0] = 127;
        vm.output().unwrap();
        vm.cells[0] = 255;
        vm.output().unwrap();
        assert_eq!(vec![127, 255], out);
    }

    #[test]
    fn test_input() {
        let mut out = vec![];
        let mut input = "aA".as_bytes();
        let mut vm = VM::new(&mut input, &mut out);
        vm.input().unwrap();
        vm.right_shift().unwrap();
        vm.input().unwrap();
        assert_eq!(vec![97, 65], vm.cells[0..2]);
    }
}
