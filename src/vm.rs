use std::io;
use std::io::Read;
use command::Command;


type Cell = u8;

pub struct VM {
    cells: [Cell; 30_000],
    data_pointer: usize,
}


impl VM {
    pub fn new() -> VM {
        VM { cells: [0; 30_000], data_pointer: 0 }
    }


    pub fn output(&self) {
        print!("{}", self.cells[self.data_pointer] as char);
    }


    pub fn input(&self) {
        let mut buffer = [0; 1];
        let mut stdin = io::stdin();
        stdin.read_exact(&mut buffer).unwrap();
    }


    pub fn increment(&mut self) {
        self.cells[self.data_pointer] = self.cells[self.data_pointer] + 1;
    }


    pub fn decrement(&mut self) {
        self.cells[self.data_pointer] = self.cells[self.data_pointer] - 1;
    }


    pub fn right_shift(&mut self) {
        self.data_pointer = self.data_pointer + 1;
    }


    pub fn left_shift(&mut self) {
        self.data_pointer = self.data_pointer - 1;
    }


    pub fn apply(&mut self, command: &Command) {
        match command {
            &Command::Output => self.output(),
            &Command::Input => self.input(),
            &Command::Increment => self.increment(),
            &Command::Decrement => self.decrement(),
            &Command::RightShift => self.right_shift(),
            &Command::LeftShift => self.left_shift(),
        }
    }


    pub fn apply_many(&mut self, commands: &[Command]) {
        for command in commands {
            self.apply(&command)
        }
    }
}
