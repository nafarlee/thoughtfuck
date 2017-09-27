use std::io;
use std::io::Read;
use command::Command;


type Cell = u8;

pub struct VM {
    pub cells: [Cell; 30_000],
    pub data_pointer: usize,
}


impl VM {
    pub fn new() -> VM {
        VM { cells: [0; 30_000], data_pointer: 0 }
    }


    pub fn output(&self) {
        print!("{}", self.cells[self.data_pointer] as char);
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
        let vm = VM::new();
        assert!(vm.cells[0] == 0);
        assert!(vm.cells[30_000 - 1] == 0);
        assert!(vm.cells[0] == 0);
    }


    #[test]
    fn test_increment() {
        let mut vm = VM::new();
        vm.increment();
        assert!(vm.cells[0] == 1);
    }
}
