use std::collections::HashMap;
use std::error::Error;

use crate::command::Command;
use crate::errors::ImbalancedBrackets;
use crate::vm::VM;

pub struct Program {
    instructions: Vec<Command>,
    instruction_pointer: Option<usize>,
    jump_stack: Vec<usize>,
    jump_table: HashMap<usize, usize>,
    status: ProgramStatus,
}

#[derive(Clone, Copy, PartialEq, Debug)]
pub enum ProgramStatus {
    Normal,
    Seeking(usize),
}

#[derive(Clone, Copy, Debug, PartialEq)]
enum StackPatch {
    Pop,
    Push(usize),
}

#[derive(Clone, Copy, Debug, PartialEq)]
struct Plan {
    pub instruction_pointer: usize,
    pub jump_stack: Option<StackPatch>,
    pub jump_table: Option<(usize, usize)>,
    pub status: ProgramStatus,
    pub command: Option<Command>,
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

    pub fn stack_depth(&self) -> usize {
        self.jump_stack.len()
    }

    pub fn assert_brackets_match(&self) -> Result<(), Box<dyn Error>> {
        let count: i64 = self.instructions.iter().fold(0, |acc, i| match i {
            Command::JumpForward => acc + 1,
            Command::JumpBackward => acc - 1,
            _ => acc,
        });
        match count {
            0 => Ok(()),
            _ => Err(ImbalancedBrackets.into()),
        }
    }

    fn plan(&self, vm: &VM) -> Option<Plan> {
        let index = self.instruction_pointer?;
        match (self.instructions[index], self.status) {
            (Command::JumpForward, ProgramStatus::Seeking(goal)) => Some(Plan {
                status: ProgramStatus::Seeking(goal),
                instruction_pointer: index + 1,
                jump_stack: Some(StackPatch::Push(index)),
                jump_table: None,
                command: None,
            }),

            (Command::JumpBackward, ProgramStatus::Seeking(goal)) => {
                let start = self
                    .jump_stack
                    .last()
                    .expect("Found ']' without a matching '['");
                let status = if goal + 1 == self.jump_stack.len() {
                    ProgramStatus::Normal
                } else {
                    ProgramStatus::Seeking(goal)
                };
                Some(Plan {
                    status: status,
                    instruction_pointer: index + 1,
                    jump_stack: Some(StackPatch::Pop),
                    jump_table: Some((*start, index)),
                    command: None,
                })
            }

            (Command::JumpForward, ProgramStatus::Normal) => {
                match (vm.get(), self.jump_table.get(&index)) {
                    (0, Some(end)) => Some(Plan {
                        status: ProgramStatus::Normal,
                        instruction_pointer: end + 1,
                        jump_stack: None,
                        jump_table: None,
                        command: None,
                    }),
                    (0, None) => Some(Plan {
                        status: ProgramStatus::Seeking(self.jump_stack.len()),
                        instruction_pointer: index + 1,
                        jump_stack: Some(StackPatch::Push(index)),
                        jump_table: None,
                        command: None,
                    }),
                    (_, _) => Some(Plan {
                        status: ProgramStatus::Normal,
                        instruction_pointer: index + 1,
                        jump_stack: Some(StackPatch::Push(index)),
                        jump_table: None,
                        command: None,
                    }),
                }
            }

            (Command::JumpBackward, ProgramStatus::Normal) => {
                let start = self
                    .jump_stack
                    .last()
                    .expect("Found ']' without a matching '['");
                match vm.get() {
                    0 => Some(Plan {
                        status: ProgramStatus::Normal,
                        instruction_pointer: index + 1,
                        jump_stack: Some(StackPatch::Pop),
                        jump_table: Some((*start, index)),
                        command: None,
                    }),
                    _ => Some(Plan {
                        status: ProgramStatus::Normal,
                        instruction_pointer: start + 1,
                        jump_stack: None,
                        jump_table: Some((*start, index)),
                        command: None,
                    }),
                }
            }

            (command, ProgramStatus::Normal) => Some(Plan {
                status: ProgramStatus::Normal,
                instruction_pointer: index + 1,
                jump_stack: None,
                jump_table: None,
                command: Some(command),
            }),

            (_, ProgramStatus::Seeking(goal)) => Some(Plan {
                status: ProgramStatus::Seeking(goal),
                instruction_pointer: index + 1,
                jump_stack: None,
                jump_table: None,
                command: None,
            }),
        }
    }

    pub fn execute(&mut self, vm: &mut VM) -> Result<(), Box<dyn Error>> {
        let Some(mut index) = self.instruction_pointer else {
            return Ok(());
        };
        while index < self.instructions.len() {
            let Some(plan) = self.plan(vm) else {
                return Ok(());
            };
            self.apply(vm, plan)?;
            index = self.instruction_pointer.unwrap();
        }
        Ok(())
    }

    fn apply(&mut self, vm: &mut VM, plan: Plan) -> Result<(), Box<dyn Error>> {
        if let Some(command) = plan.command {
            vm.apply(command)?;
        };
        self.status = plan.status;
        self.instruction_pointer = Some(plan.instruction_pointer);
        match plan.jump_stack {
            None => {}
            Some(StackPatch::Pop) => {
                self.jump_stack.pop();
            }
            Some(StackPatch::Push(index)) => {
                self.jump_stack.push(index);
            }
        };
        match plan.jump_table {
            None => {}
            Some((start, end)) => {
                self.jump_table.insert(start, end);
            }
        };
        Ok(())
    }
}
