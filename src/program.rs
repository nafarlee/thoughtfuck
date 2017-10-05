use command::Command;
use vm::VM;
use vm::Cell;


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


#[derive(Clone, Copy, Debug, PartialEq)]
pub struct ProgramPatch {
    pub instruction_pointer: usize,
    pub current_depth: u64,
    pub status: ProgramStatus,
}


fn find_by<T, F>(list: &[T], start: Option<usize>, mut predicate: F) -> Option<usize>
where
    F: FnMut(&T) -> bool,
{
    let start = start.unwrap_or(0);
    for (offset, element) in list.iter().enumerate() {
        if predicate(element) {
            return Some(start + offset);
        }
    }

    return None;
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
        if let Some(mut index) = self.instruction_pointer {
            while index < self.instructions.len() {
                let current_cell = vm.cells[vm.data_pointer];
                let patch = match (self.instructions[index], self.status) {
                    (_, ProgramStatus::Seeking(goal)) => {
                        Program::forward_jump(
                            &self.instructions,
                            index,
                            goal,
                            self.current_depth,
                        )
                    }

                    (Command::JumpForward, _) => self.process_jump_forward(index, current_cell),

                    (Command::JumpBackward, _) => {
                        match current_cell {
                            0 => ProgramPatch {
                                instruction_pointer: index + 1,
                                status: ProgramStatus::Normal,
                                current_depth: self.current_depth - 1,
                            },

                            _ => {
                                Program::backward_jump(
                                    &self.instructions,
                                    index,
                                    self.current_depth,
                                )
                            }
                        }
                    }

                    (command, _) => {
                        vm.apply(command);
                        ProgramPatch {
                            status: ProgramStatus::Normal,
                            current_depth: self.current_depth,
                            instruction_pointer: index + 1,
                        }
                    }
                };

                self.update(patch);
                index = patch.instruction_pointer;
            }
        }
    }


    fn process_jump_forward(&self, index: usize, current_cell: Cell) -> ProgramPatch {
        match current_cell {
            0 => {
                Program::forward_jump(
                    &self.instructions,
                    index,
                    self.current_depth,
                    self.current_depth,
                )
            }

            _ => ProgramPatch {
                instruction_pointer: index + 1,
                status: ProgramStatus::Normal,
                current_depth: self.current_depth + 1,
            },
        }
    }


    fn update(&mut self, patch: ProgramPatch) -> &mut Self {
        self.instruction_pointer = Some(patch.instruction_pointer);
        self.current_depth = patch.current_depth;
        self.status = patch.status;
        return self;
    }


    fn backward_jump(commands: &Vec<Command>, start: usize, depth: u64) -> ProgramPatch {
        let goal_depth = depth - 1;
        let mut current_depth = depth;
        for index in (0..start).rev() {
            match commands[index] {
                Command::JumpBackward => current_depth += 1,
                Command::JumpForward => current_depth -= 1,
                _ => {}
            }
            if goal_depth == current_depth {
                return ProgramPatch {
                    instruction_pointer: index + 1,
                    status: ProgramStatus::Normal,
                    current_depth: current_depth + 1,
                };
            }
        }
        panic!("No opening bracket found?");
    }


    fn forward_jump(
        commands: &Vec<Command>,
        index: usize,
        goal_depth: u64,
        mut current_depth: u64,
    ) -> ProgramPatch {
        let index = find_by(&commands[index..], Some(index), |command| {
            match command {
                &Command::JumpBackward => current_depth -= 1,
                &Command::JumpForward => current_depth += 1,
                _ => {}
            };
            return current_depth == goal_depth;
        });

        return match index {
            Some(index) => ProgramPatch {
                instruction_pointer: index + 1,
                status: ProgramStatus::Normal,
                current_depth,
            },

            None => ProgramPatch {
                instruction_pointer: commands.len(),
                status: ProgramStatus::Seeking(goal_depth),
                current_depth,
            },
        };
    }
}


#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn attempt_forward_jump() {
        let commands = vec![
            Command::JumpForward,
            Command::JumpForward,
            Command::JumpBackward,
        ];
        let index = 0;
        let goal_depth = 0;
        let current_depth = 0;
        let actual = Program::attempt_forward_jump(&commands, index, goal_depth, current_depth);
        assert_eq!(
            actual,
            ProgramPatch {
                instruction_pointer: 3,
                current_depth: 1,
                status: ProgramStatus::Seeking(0),
            }
        )
    }
}
