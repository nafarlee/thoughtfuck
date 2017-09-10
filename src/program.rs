use command::Command;
use vm::VM;


pub struct Program {
    instructions : Vec<Command>,
    instruction_pointer: Option<usize>,
    pub is_seeking: bool,
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
                if self.is_seeking {
                    let (new_index, still_seeking) = self.seek_forward(index);
                    self.is_seeking = still_seeking;
                    index = new_index;
                }
                while index < self.instructions.len() {
                    let command = self.instructions[index];
                    if command == Command::JumpForward {
                        index = self.start_jump_forward(vm, index);
                    } else if command == Command::JumpBackward {
                        self.current_depth = self.current_depth - 1
                    } else {
                        vm.apply(command);
                        index = index + 1;
                    }
                }
                self.instruction_pointer = Some(index);
            }
        }
    }


    fn jump_backward (&mut self, vm: &VM, starting_index: usize) -> usize {
        match vm.cells[vm.data_pointer] {
            0 => {
                self.current_depth = self.current_depth - 1;
                starting_index + 1
            },
            _ => {
                let goal_depth = self.current_depth;
                for index in (0..starting_index).rev() {
                    match self.instructions[index] {
                        Command::JumpBackward => {
                            self.current_depth = self.current_depth + 1;
                        },
                        Command::JumpForward => {
                            if self.current_depth == goal_depth { return index + 1 }
                            self.current_depth = self.current_depth - 1;
                        },
                        _ => {},
                    }
                }
                panic!("No starting brace found!");
            },
        }
    }


    fn seek_forward(&mut self, starting_index: usize) -> (usize, bool) {
        let goal_depth = self.goal_depth.unwrap();
        for index in starting_index..self.instructions.len() {
            match self.instructions[index] {
                Command::JumpForward => self.current_depth = self.current_depth + 1,
                Command::JumpBackward => self.current_depth = self.current_depth - 1,
                _ => {},
            }
            if self.current_depth == goal_depth { return (index + 1, false) }
        }
        (self.instructions.len(), true)
    }


    fn start_jump_forward(&mut self, vm: &VM, index: usize) -> usize {
        if vm.cells[vm.data_pointer] == 0 {
            self.goal_depth = Some(self.current_depth);
            let (index, still_seeking) = self.seek_forward(index);
            self.is_seeking = still_seeking;
            return index;
        } else {
            self.current_depth = self.current_depth + 1;
            return index + 1;
        }
    }
}
