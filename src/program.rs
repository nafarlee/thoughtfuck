use std::collections::HashMap;
use std::error::Error;

use crate::command::Command;
use crate::errors::ImbalancedBrackets;
use crate::vm::VM;

#[derive(Default)]
pub struct Program {
    instructions: Vec<Command>,
    instruction_pointer: Option<usize>,
    jump_stack: Vec<usize>,
    jump_table: HashMap<usize, usize>,
    status: ProgramStatus,
}

#[derive(Clone, Copy, PartialEq, Debug, Default)]
pub enum ProgramStatus {
    #[default]
    Normal,
    Seeking(usize),
}

#[derive(Clone, Copy, Debug, PartialEq)]
enum Effect {
    StartSeeking,
    Push,
    Pop,
    FindRightBracket,
    JumpToRightBracket,
    JumpToLeftBracket,
    Apply(Command),
    Seek,
    NoOp,
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
        let count = self.instructions.iter().try_fold(0, |acc, i| match i {
            Command::JumpForward => Ok(acc + 1),
            Command::JumpBackward if acc == 0 => Err(ImbalancedBrackets),
            Command::JumpBackward => Ok(acc - 1),
            _ => Ok(acc),
        })?;
        match count {
            0 => Ok(()),
            _ => Err(ImbalancedBrackets.into()),
        }
    }

    fn plan(&self, vm: &VM) -> Result<Effect, Box<dyn Error>> {
        let Some(index) = self.instruction_pointer else {
            return Ok(Effect::NoOp);
        };
        match (self.instructions[index], self.status) {
            (Command::JumpForward, ProgramStatus::Seeking(_)) => Ok(Effect::Push),
            (Command::JumpBackward, ProgramStatus::Seeking(goal)) => {
                if goal + 1 == self.stack_depth() {
                    Ok(Effect::FindRightBracket)
                } else {
                    Ok(Effect::Pop)
                }
            }
            (Command::JumpForward, ProgramStatus::Normal) => {
                match (vm.get(), self.jump_table.get(&index)) {
                    (0, Some(_)) => Ok(Effect::JumpToRightBracket),
                    (0, None) => Ok(Effect::StartSeeking),
                    (_, _) => Ok(Effect::Push),
                }
            }
            (Command::JumpBackward, ProgramStatus::Normal) => match vm.get() {
                0 => Ok(Effect::Pop),
                _ => Ok(Effect::JumpToLeftBracket),
            },
            (command, ProgramStatus::Normal) => Ok(Effect::Apply(command)),
            (_, ProgramStatus::Seeking(_)) => Ok(Effect::Seek),
        }
    }

    pub fn execute(&mut self, vm: &mut VM) -> Result<(), Box<dyn Error>> {
        let Some(mut index) = self.instruction_pointer else {
            return Ok(());
        };
        while index < self.instructions.len() {
            let plan = self.plan(vm)?;
            self.apply(vm, plan)?;
            index = self.instruction_pointer.unwrap();
        }
        Ok(())
    }

    fn apply(&mut self, vm: &mut VM, effect: Effect) -> Result<(), Box<dyn Error>> {
        let Some(index) = self.instruction_pointer else {
            return Ok(());
        };
        match effect {
            Effect::StartSeeking => {
                self.status = ProgramStatus::Seeking(self.stack_depth());
                self.jump_stack.push(index);
                self.instruction_pointer = Some(index + 1);
            }
            Effect::Push => {
                self.jump_stack.push(index);
                self.instruction_pointer = Some(index + 1);
            }
            Effect::Pop => {
                let Some(start) = self.jump_stack.pop() else {
                    return Err(ImbalancedBrackets.into());
                };
                self.jump_table.insert(start, index);
                self.instruction_pointer = Some(index + 1);
            }
            Effect::FindRightBracket => {
                let Some(start) = self.jump_stack.pop() else {
                    return Err(ImbalancedBrackets.into());
                };
                self.status = ProgramStatus::Normal;
                self.jump_table.insert(start, index);
                self.instruction_pointer = Some(index + 1);
            }
            Effect::JumpToRightBracket => {
                let Some(end) = self.jump_table.get(&index) else {
                    return Err(ImbalancedBrackets.into());
                };
                self.instruction_pointer = Some(end + 1);
            }
            Effect::JumpToLeftBracket => {
                let Some(start) = self.jump_stack.last() else {
                    return Err(ImbalancedBrackets.into());
                };
                self.instruction_pointer = Some(start + 1)
            }
            Effect::Seek => {
                self.instruction_pointer = Some(index + 1);
            }
            Effect::Apply(c) => {
                vm.apply(c)?;
                self.instruction_pointer = Some(index + 1);
            }
            Effect::NoOp => {}
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_imbalanced_bracket() {
        let mut program = Program::new();
        program.append(&vec![Command::JumpForward]);
        let Err(err) = program.assert_brackets_match() else {
            panic!();
        };
        assert_eq!(
            Some(&ImbalancedBrackets),
            err.downcast_ref::<ImbalancedBrackets>()
        );
    }

    #[test]
    fn test_reverse_balanced_brackets() {
        let mut program = Program::new();
        program.append(&vec![Command::JumpBackward, Command::JumpForward]);
        let Err(err) = program.assert_brackets_match() else {
            panic!();
        };
        assert_eq!(
            Some(&ImbalancedBrackets),
            err.downcast_ref::<ImbalancedBrackets>()
        );
    }
}
