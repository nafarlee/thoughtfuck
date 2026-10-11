use crate::command::Command;
use std::io::{Read, Write};

pub type Cell = u8;

pub struct VM<'a> {
    cells: [Cell; 30_000],
    data_pointer: usize,
    out: &'a mut dyn Write,
    input: &'a mut dyn Read,
}

impl<'a> VM<'a> {
    pub fn new(input: &'a mut dyn Read, out: &'a mut dyn Write) -> VM<'a> {
        VM {
            cells: [0; 30_000],
            data_pointer: 0,
            out,
            input,
        }
    }

    pub fn get(&self) -> Cell {
        return self.cells[self.data_pointer];
    }

    pub fn output(&mut self) {
        self.out
            .write_all(&[self.get()])
            .expect("Could not output current byte");
    }

    pub fn input(&mut self) {
        match self.input.bytes().next() {
            Some(res) => match res {
                Ok(value) => self.cells[self.data_pointer] = value,
                Err(error) => panic!("{}", error),
            },
            None => {}
        }
    }

    pub fn increment(&mut self) {
        self.cells[self.data_pointer] = self.get().checked_add(1).unwrap_or(0);
    }

    pub fn decrement(&mut self) {
        self.cells[self.data_pointer] = self.get().checked_sub(1).unwrap_or(Cell::MAX);
    }

    pub fn right_shift(&mut self) {
        self.data_pointer += 1;
    }

    pub fn left_shift(&mut self) {
        self.data_pointer -= 1;
    }

    pub fn apply(&mut self, command: Command) {
        match command {
            Command::Output => self.output(),
            Command::Input => self.input(),
            Command::Increment => self.increment(),
            Command::Decrement => self.decrement(),
            Command::RightShift => self.right_shift(),
            Command::LeftShift => self.left_shift(),
            _ => {}
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
        assert!(vm.cells[30_000 - 1] == 0);
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
        vm.right_shift();
        assert!(vm.data_pointer == 1);
    }

    #[test]
    fn test_left_shift() {
        const STARTING_POSITION: usize = 2;

        let mut stdout = io::stdout();
        let mut input = "".as_bytes();
        let mut vm = VM::new(&mut input, &mut stdout);
        vm.data_pointer = STARTING_POSITION;
        vm.left_shift();
        assert!(vm.data_pointer == 1);
    }

    #[test]
    fn test_output() {
        let mut out = vec![];
        let mut input = "".as_bytes();
        let mut vm = VM::new(&mut input, &mut out);
        vm.cells[0] = 127;
        vm.output();
        vm.cells[0] = 255;
        vm.output();
        assert_eq!(vec![127, 255], out);
    }

    #[test]
    fn test_input() {
        let mut out = vec![];
        let mut input = "aA".as_bytes();
        let mut vm = VM::new(&mut input, &mut out);
        vm.input();
        vm.right_shift();
        vm.input();
        assert_eq!(vec![97, 65], vm.cells[0..2]);
    }
}
