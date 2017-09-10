use command::Command;
use vm::VM;


pub struct Program {
    instructions : Vec<Command>,
    instruction_pointer: Option<usize>,
    is_seeking: bool,
    current_depth: u64,
    goal_depth: Option<u64>,
}


impl Program {
    pub fn new () -> Program {
        Program {
            instructions: Vec::new(),
            instruction_pointer: None,
            is_seeking: false,
            current_depth: 0,
            goal_depth: None,
        }
    }


    pub fn append(&mut self, instructions: &[Command]) {
        self.instructions.extend(instructions.iter().cloned());
        if self.instruction_pointer.is_none() { self.instruction_pointer = Some(0); }
    }


    pub fn execute(&mut self, vm: &mut VM) {
        match self.instruction_pointer {
            None => {},
            Some(mut index) => {
                while index < self.instructions.len() {
                    let command = self.instructions[index];
                    if command == Command::JumpForward { self.current_depth = self.current_depth + 1}
                    if command == Command::JumpBackward { self.current_depth = self.current_depth - 1}
                    vm.apply(command);
                    index = index + 1;
                }
                self.instruction_pointer = Some(index);
            }
        }
    }


    fn seek_forward(&mut self, mut index: usize) -> usize {
        while self.current_depth != self.goal_depth.unwrap() && index < self.instructions.len() {
            match self.instructions[index] {
                Command::JumpForward => self.current_depth = self.current_depth + 1,
                Command::JumpBackward => self.current_depth = self.current_depth - 1,
                _ => {},
            }
            index = index + 1;
        }
        return index;
    }


    fn handle_jump_forward(&mut self, vm: &VM, index: usize) -> usize {
        if vm.cells[vm.data_pointer] == 0 {
            self.goal_depth = Some(self.current_depth);
            return self.seek_forward(index)
        } else {
            self.current_depth = self.current_depth + 1;
            return index;
        }
    }
}
