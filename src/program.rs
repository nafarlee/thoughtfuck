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


fn find_by<T, F>(list: &[T], predicate: F, start: Option<usize>) -> Option<usize> where F: Fn(&T) -> bool {
    let start = start.unwrap_or(0);
    for (offset, element) in list.iter().enumerate() {
        if predicate(element) { return Some(start + offset); }
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
        match (self.instruction_pointer, self.status) {
            (Some(index), ProgramStatus::Seeking(goal)) if index < self.instructions.len() => {}

            (Some(index), ProgramStatus::Normal) if index < self.instructions.len() => {}

            _ => {}
        }
    }
}
