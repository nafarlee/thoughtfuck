use command::Command;
use vm::VM;


pub struct Program {
    instructions : Vec<Command>,
    instruction_pointer: Option<usize>,
    current_depth: u64,
    pub status: ProgramStatus,
}

enum ProgramStatus {
    Normal,
    Seeking(u64),
}


impl Program {
    pub fn new () -> Program {
        Program {
            instructions: Vec::new(),
            instruction_pointer: None,
            current_depth: 0,
            status: ProgramStatus::Normal,
        }
    }


    pub fn append(&mut self, instructions: &[Command]) {
        self.instructions.extend(instructions.iter().cloned());
        if self.instruction_pointer.is_none() { self.instruction_pointer = Some(0); }
    }


    pub fn execute(&mut self, vm: &mut VM) {
        if let Some(mut index) = self.instruction_pointer {
            match self.status {
                ProgramStatus::Normal => {
                    while index < self.instructions.len() {
                        match self.instructions[index] {
                            Command::JumpForward => {
                                let (tmp_index, status) = self.start_jump_forward(vm, index);
                                self.status = status;
                                index = tmp_index;
                            },
                            Command::JumpBackward => index = self.jump_backward(&vm, index),
                            command => {
                                vm.apply(command);
                                index += 1;
                            }
                        }
                        self.instruction_pointer = Some(index);
                    }
                },
                ProgramStatus::Seeking(goal_depth) => {
                    let (new_index, new_status) = self.seek_forward(index, goal_depth);
                    self.instruction_pointer = Some(new_index);
                    self.status = new_status;
                    self.execute(vm);
                }
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
        match vm.cells[vm.data_pointer] {
            0 => {
                self.status = ProgramStatus::Seeking(self.current_depth);
                let (index, still_seeking) = self.seek_forward(index);
                self.status = if still_seeking { self.status } else { ProgramStatus::Normal };
                index
            },
            _ => {
                self.current_depth = self.current_depth + 1;
                index + 1
            }
        }
    }
}
