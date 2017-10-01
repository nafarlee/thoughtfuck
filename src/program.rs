use command::Command;
use vm::VM;


pub struct Program {
    instructions : Vec<Command>,
    instruction_pointer: Option<usize>,
    current_depth: u64,
    pub status: ProgramStatus,
}

#[derive(Clone,Copy,PartialEq,Debug)]
pub enum ProgramStatus {
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
        match (self.instruction_pointer, self.status) {
            (Some(mut index), ProgramStatus::Normal) => {
                while index < self.instructions.len() {
                    match self.instructions[index] {
                        Command::JumpForward => {
                            let jump_report = self.start_jump_forward(vm, index);
                            index = jump_report.0;
                            self.status = jump_report.1;
                        },
                        Command::JumpBackward => index = self.jump_backward(&vm, index),
                        command => {
                            vm.apply(command);
                            index += 1;
                        }
                    }
                }
                self.instruction_pointer = Some(index);
            },

            (Some(index), ProgramStatus::Seeking(goal_depth)) => {
                let (new_index, new_status) = self.seek_forward(index, goal_depth);
                self.instruction_pointer = Some(new_index);
                self.status = new_status;
                self.execute(vm);
            },

            _ => {}
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
                        Command::JumpBackward => self.current_depth += 1,
                        Command::JumpForward => {
                            if self.current_depth == goal_depth { return index + 1 }
                            self.current_depth -= 1;
                        },
                        _ => {},
                    }
                }
                panic!("No starting brace found!");
            },
        }
    }


    fn seek_forward(&mut self, starting_index: usize, goal_depth: u64) -> (usize, ProgramStatus) {
        for index in starting_index..self.instructions.len() {
            match self.instructions[index] {
                Command::JumpForward => self.current_depth += 1,
                Command::JumpBackward => self.current_depth -= 1,
                _ => {},
            }
            if self.current_depth == goal_depth {
                return (index + 1, ProgramStatus::Normal);
            }
        }
        return (self.instructions.len(), ProgramStatus::Seeking(goal_depth));
    }


    fn start_jump_forward(&mut self, vm: &VM, index: usize) -> (usize, ProgramStatus) {
        match vm.cells[vm.data_pointer] {
            0 => {
                let goal_depth = self.current_depth;
                return self.seek_forward(index, goal_depth);
            },
            _ => {
                self.current_depth += 1;
                return (index + 1, ProgramStatus::Normal);
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    pub fn test_seek_forward_normal() {
        let mut program = Program {
            instruction_pointer: Some(0),
            current_depth: 0,
            status: ProgramStatus::Normal,
            instructions: vec![ Command::JumpForward
                              , Command::JumpForward
                              , Command::JumpBackward
                              , Command::JumpBackward]};
        let actual = program.seek_forward(0, 0);
        let expected = (4, ProgramStatus::Normal);
        assert_eq!(actual, expected);
    }


    #[test]
    pub fn test_seek_forward_seeking() {
        let mut program = Program {
            instruction_pointer: Some(0),
            current_depth: 0,
            status: ProgramStatus::Normal,
            instructions: vec![ Command::JumpForward
                              , Command::JumpForward
                              , Command::JumpBackward]};
        let actual = program.seek_forward(0, 0);
        let expected = (3, ProgramStatus::Seeking(0));
        assert_eq!(actual, expected);
    }
}
