use std::io;
use std::io::{Write,Read};
use command::Command;


type Cell = u8;

pub struct VM <'a> {
    pub cells: [Cell; 30_000],
    pub data_pointer: usize,
    out: &'a mut Write
}


impl <'a> VM <'a> {
    pub fn new(out: &mut Write) -> VM {
        VM { cells: [0; 30_000], data_pointer: 0, out }
    }


    pub fn output(&mut self) {
        self.out
            .write_fmt(format_args!("{}", self.cells[self.data_pointer] as char))
            .expect("Could not output current byte");
    }


    pub fn input(&mut self) {
        match io::stdin().bytes().next() {
            Some(res) => match res {
                Ok(value) => self.cells[self.data_pointer] = value,
                Err(error) => panic!(error),
            },
            None => {}
        }
        println!("");
    }


    pub fn increment(&mut self) {
        self.cells[self.data_pointer] = self.cells[self.data_pointer].checked_add(1).unwrap_or(0);
    }


    pub fn decrement(&mut self) {
        self.cells[self.data_pointer] = self.cells[self.data_pointer]
            .checked_sub(1)
            .unwrap_or(Cell::max_value());
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
            _ => {},
        }
    }
}


#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_new() {
        let vm = VM::new(None);
        assert!(vm.cells[0] == 0);
        assert!(vm.cells[30_000 - 1] == 0);
        assert!(vm.cells[0] == 0);
    }


    #[test]
    fn test_increment() {
        let mut vm = VM::new(None);
        vm.increment();
        assert!(vm.cells[0] == 1);
    }


    #[test]
    fn test_decrement() {
        const STARTING_VALUE: u8 = 2;

        let mut vm = VM::new(None);
        vm.cells[0] = STARTING_VALUE;
        vm.decrement();
        assert!(vm.cells[0] == STARTING_VALUE - 1);
    }


    #[test]
    fn test_right_shift() {
        let mut vm = VM::new(None);
        vm.right_shift();
        assert!(vm.data_pointer == 1);
    }


    #[test]
    fn test_left_shift() {
        const STARTING_POSITION: usize = 2;

        let mut vm = VM::new(None);
        vm.data_pointer  = STARTING_POSITION;
        vm.left_shift();
        assert!(vm.data_pointer == 1);
    }
}
