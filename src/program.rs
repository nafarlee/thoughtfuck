use std::collections::HashMap;
use std::error::Error;

use crate::command::Command;
use crate::errors::ImbalancedBrackets;
use crate::vm::VM;

#[derive(Default)]
pub struct Program {
    instructions: Vec<Command>,
    instruction_pointer: usize,
    jump_stack: Vec<usize>,
    jump_table: HashMap<usize, usize>,
    status: ProgramStatus,
}

#[derive(Clone, Copy, PartialEq, Debug, Default)]
enum ProgramStatus {
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
}

impl Program {
    pub fn new() -> Program {
        Self::default()
    }

    pub fn append(&mut self, instructions: &[Command]) {
        self.instructions.extend(instructions.iter().cloned());
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

    fn plan(&self, vm: &VM) -> Effect {
        let index = self.instruction_pointer;
        match (self.instructions[index], self.status) {
            (Command::JumpForward, ProgramStatus::Seeking(_)) => Effect::Push,
            (Command::JumpBackward, ProgramStatus::Seeking(goal)) => {
                if goal + 1 == self.stack_depth() {
                    Effect::FindRightBracket
                } else {
                    Effect::Pop
                }
            }
            (Command::JumpForward, ProgramStatus::Normal) => {
                match (vm.get(), self.jump_table.get(&index)) {
                    (0, Some(_)) => Effect::JumpToRightBracket,
                    (0, None) => Effect::StartSeeking,
                    (_, _) => Effect::Push,
                }
            }
            (Command::JumpBackward, ProgramStatus::Normal) => match vm.get() {
                0 => Effect::Pop,
                _ => Effect::JumpToLeftBracket,
            },
            (command, ProgramStatus::Normal) => Effect::Apply(command),
            (_, ProgramStatus::Seeking(_)) => Effect::Seek,
        }
    }

    pub fn execute(&mut self, vm: &mut VM) -> Result<(), Box<dyn Error>> {
        let mut index = self.instruction_pointer;
        while index < self.instructions.len() {
            let plan = self.plan(vm);
            self.apply(vm, plan)?;
            index = self.instruction_pointer
        }
        Ok(())
    }

    fn apply(&mut self, vm: &mut VM, effect: Effect) -> Result<(), Box<dyn Error>> {
        let index = self.instruction_pointer;
        match effect {
            Effect::StartSeeking => {
                self.status = ProgramStatus::Seeking(self.stack_depth());
                self.jump_stack.push(index);
                self.instruction_pointer = index + 1;
            }
            Effect::Push => {
                self.jump_stack.push(index);
                self.instruction_pointer = index + 1;
            }
            Effect::Pop => {
                let Some(start) = self.jump_stack.pop() else {
                    return Err(ImbalancedBrackets.into());
                };
                self.jump_table.insert(start, index);
                self.instruction_pointer = index + 1;
            }
            Effect::FindRightBracket => {
                let Some(start) = self.jump_stack.pop() else {
                    return Err(ImbalancedBrackets.into());
                };
                self.status = ProgramStatus::Normal;
                self.jump_table.insert(start, index);
                self.instruction_pointer = index + 1;
            }
            Effect::JumpToRightBracket => {
                let Some(end) = self.jump_table.get(&index) else {
                    return Err(ImbalancedBrackets.into());
                };
                self.instruction_pointer = end + 1;
            }
            Effect::JumpToLeftBracket => {
                let Some(start) = self.jump_stack.last() else {
                    return Err(ImbalancedBrackets.into());
                };
                self.instruction_pointer = start + 1
            }
            Effect::Seek => {
                self.instruction_pointer = index + 1;
            }
            Effect::Apply(c) => {
                vm.apply(c)?;
                self.instruction_pointer = index + 1;
            }
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
