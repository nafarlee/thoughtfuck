use crate::command::Command;
use crate::vm::VM;
use std::collections::HashMap;


pub struct Program {
    instructions: Vec<Command>,
    instruction_pointer: Option<usize>,
    jump_stack: Vec<usize>,
    jump_table: HashMap<usize, usize>,
    pub status: ProgramStatus,
}


#[derive(Clone, Copy, PartialEq, Debug)]
pub enum ProgramStatus {
    Normal,
    Seeking(usize),
}


#[derive(Clone, Copy, Debug, PartialEq)]
pub enum StackPatch {
    Pop,
    Push(usize),
}


#[derive(Clone, Copy, Debug, PartialEq)]
pub struct ProgramPatch {
    pub instruction_pointer: usize,
    pub jump_stack: Option<StackPatch>,
    pub jump_table: Option<(usize, usize)>,
    pub status: ProgramStatus,
}


impl Program {
    pub fn new() -> Program {
        Program {
            instructions: Vec::new(),
            instruction_pointer: None,
            jump_stack: Vec::new(),
            jump_table: HashMap::new(),
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
                    (Command::JumpForward, ProgramStatus::Seeking(goal)) => {
                        ProgramPatch {
                            status: ProgramStatus::Seeking(goal),
                            instruction_pointer: index + 1,
                            jump_stack: Some(StackPatch::Push(index)),
                            jump_table: None,
                        }
                    }

                    (Command::JumpBackward, ProgramStatus::Seeking(goal)) => {
                        let start = self.jump_stack.last().expect("Found ']' without a matching '['");
                        let status = if goal + 1 == self.jump_stack.len() {
                            ProgramStatus::Normal
                        } else {
                            ProgramStatus::Seeking(goal)
                        };
                        ProgramPatch {
                            status: status,
                            instruction_pointer: index + 1,
                            jump_stack: Some(StackPatch::Pop),
                            jump_table: Some((*start, index)),
                        }
                    }

                    (Command::JumpForward, ProgramStatus::Normal) => {
                        match (current_cell, self.jump_table.get(&index)) {
                            (0, Some(end)) => {
                                ProgramPatch {
                                    status: ProgramStatus::Normal,
                                    instruction_pointer: end + 1,
                                    jump_stack: None,
                                    jump_table: None,
                                }
                            }
                            (0, None) => {
                                ProgramPatch {
                                    status: ProgramStatus::Seeking(self.jump_stack.len()),
                                    instruction_pointer: index + 1,
                                    jump_stack: Some(StackPatch::Push(index)),
                                    jump_table: None,
                                }
                            }
                            (_, _) => {
                                ProgramPatch {
                                    status: ProgramStatus::Normal,
                                    instruction_pointer: index + 1,
                                    jump_stack: Some(StackPatch::Push(index)),
                                    jump_table: None,
                                }
                            }
                        }
                    }

                    (Command::JumpBackward, ProgramStatus::Normal) => {
                        let start = self.jump_stack.last().expect("Found ']' without a matching '['");
                        match current_cell {
                            0 => ProgramPatch {
                                status: ProgramStatus::Normal,
                                instruction_pointer: index + 1,
                                jump_stack: Some(StackPatch::Pop),
                                jump_table: Some((*start, index)),
                            },
                            _ => ProgramPatch {
                                status: ProgramStatus::Normal,
                                instruction_pointer: start + 1,
                                jump_stack: None,
                                jump_table: Some((*start, index)),
                            },
                        }
                    }

                    (command, ProgramStatus::Normal) => {
                        vm.apply(command);
                        ProgramPatch {
                            status: ProgramStatus::Normal,
                            instruction_pointer: index + 1,
                            jump_stack: None,
                            jump_table: None,
                        }
                    }

                    (_, ProgramStatus::Seeking(goal)) => ProgramPatch {
                        status: ProgramStatus::Seeking(goal),
                        instruction_pointer: index + 1,
                        jump_stack: None,
                        jump_table: None,
                    }
                };

                self.update(patch);
                index = patch.instruction_pointer;
            }
        }
    }


    fn update(&mut self, patch: ProgramPatch) -> &mut Self {
        self.status = patch.status;
        self.instruction_pointer = Some(patch.instruction_pointer);
        match patch.jump_stack {
            None => {},
            Some(StackPatch::Pop) => {
                self.jump_stack.pop();
            }
            Some(StackPatch::Push(index)) => {
                self.jump_stack.push(index);
            }
        };
        match patch.jump_table {
            None => {},
            Some((start, end)) => {
                self.jump_table.insert(start, end);
            }
        };
        return self;
    }
}
