use command::Command;
use vm::VM;


pub struct Program {
    instructions: Vec<Command>,
    instruction_pointer: Option<usize>,
    current_depth: u64,
    pub status: ProgramStatus,
}

#[derive(Clone, Copy, PartialEq, Debug)]
pub enum ProgramStatus {
    Normal,
    Seeking(u64),
}


impl Program {
    pub fn new() -> Program {
        Program {
            instructions: Vec::new(),
            instruction_pointer: None,
            current_depth: 0,
            status: ProgramStatus::Normal,
        }
    }


    pub fn append(&mut self, instructions: &[Command]) {
        self.instructions.extend(instructions.iter().cloned());
        if self.instruction_pointer.is_none() {
            self.instruction_pointer = Some(0);
        }
    }


    pub fn execute(&mut self, vm: &mut VM) {
        match (self.instruction_pointer, self.status) {
            (Some(mut index), ProgramStatus::Seeking(_)) => {
                self.handle_jump_forward(vm, &mut index);
                self.instruction_pointer = Some(index);
                if index < self.instructions.len() { self.execute(vm); }
            }

            (Some(mut index), ProgramStatus::Normal) => {
                while index < self.instructions.len() {
                    match self.instructions[index] {
                        Command::JumpForward => self.handle_jump_forward(vm, &mut index),
                        Command::JumpBackward => self.handle_jump_backward(vm, &mut index),
                        command => {
                            vm.apply(command);
                            index += 1;
                        }
                    }
                }

                self.instruction_pointer = Some(index);
            }

            _ => {}
        }
    }


    fn handle_jump_backward(&mut self, vm: &VM, index_ref: &mut usize) {
        match vm.cells[vm.data_pointer] {
            0 => {
                self.current_depth -= 1;
                *index_ref += 1;
            }

            _ => {
                let goal_depth = self.current_depth;
                for index in (0..*index_ref).rev() {
                    match self.instructions[index] {
                        Command::JumpBackward => self.current_depth += 1,
                        Command::JumpForward => {
                            if self.current_depth == goal_depth {
                                *index_ref = index + 1;
                                return;
                            }
                            self.current_depth -= 1;
                        }
                        _ => {}
                    }
                }
                panic!("No starting brace found!");
            }
        }
    }


    fn seek_forward(&self, starting_index: usize) -> (usize, ProgramStatus) {
        match self.status {
            ProgramStatus::Seeking(goal_depth) => {
                for index in starting_index..self.instructions.len() {
                    match self.instructions[index] {
                        Command::JumpForward => self.current_depth += 1,
                        Command::JumpBackward => self.current_depth -= 1,
                        _ => {}
                    }
                    if self.current_depth == goal_depth {
                        return (index + 1, ProgramStatus::Normal);
                    }
                }

                return (self.instructions.len(), ProgramStatus::Seeking(goal_depth));
            }

            ProgramStatus::Normal => return (starting_index, self.status)
        }
    }


    fn handle_jump_forward(&mut self, vm: &VM, index: &mut usize) {
        match (vm.cells[vm.data_pointer], self.status) {
            (0, ProgramStatus::Normal) => {
                self.status = ProgramStatus::Seeking(self.current_depth);
                self.seek_forward(index);
            }

            (_, ProgramStatus::Normal) => {
                *index += 1;
                self.current_depth += 1;
                self.status = ProgramStatus::Normal;
            }

            (_, ProgramStatus::Seeking(_)) => self.seek_forward(index),

        }
    }
}


#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    pub fn test_seek_forward_normal() {
        let mut program = Program {
            instruction_pointer: None,
            current_depth: 0,
            status: ProgramStatus::Seeking(0),
            instructions: vec![
                Command::JumpForward,
                Command::JumpForward,
                Command::JumpBackward,
                Command::JumpBackward,
            ],
        };
        let mut index = 0;
        program.seek_forward(&mut index);
        assert_eq!(index, 4);
        assert_eq!(program.status, ProgramStatus::Normal);
    }


    #[test]
    pub fn test_seek_forward_seeking() {
        let mut program = Program {
            instruction_pointer: None,
            current_depth: 0,
            status: ProgramStatus::Seeking(0),
            instructions: vec![
                Command::JumpForward,
                Command::JumpForward,
                Command::JumpBackward,
            ],
        };
        let mut index = 0;
        program.seek_forward(&mut index);
        assert_eq!(index, 3);
        assert_eq!(program.status, ProgramStatus::Seeking(0));
    }
}
