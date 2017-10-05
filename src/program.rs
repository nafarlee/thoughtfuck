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

#[derive(Clone, Copy, Debug)]
pub struct ProgramPatch {
    pub instruction_pointer: usize,
    pub current_depth: u64,
    pub status: ProgramStatus,
}


fn find_by<T, F>(list: &[T], start: Option<usize>, mut predicate: F) -> Option<usize>
where F: FnMut(&T) -> bool {
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
            (Some(index), ProgramStatus::Seeking(goal)) if index < self.instructions.len() => {

            }

            (Some(start), ProgramStatus::Normal) if start < self.instructions.len() => {
                for index in start..self.instructions.len() {
                    match self.instructions[index] {
                        Command::JumpBackward => {},

                        Command::JumpForward => {
                            let patch = Program::attempt_forward_jump(&self.instructions, index, self.current_depth);
                            return self.update(patch).execute(vm);
                        },

                        command => vm.apply(command),
                    }
                }
            }

            _ => {}
        }
    }


    fn update (&mut self, patch: ProgramPatch) -> &mut Self {
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
                _ => {},
            }
            if goal_depth == current_depth {
                return ProgramPatch { instruction_pointer: index, status: ProgramStatus::Normal, current_depth };
            }
        }
        panic!("No opening bracket found?");
    }


    fn attempt_forward_jump(commands: &Vec<Command>, index: usize, goal_depth: u64, mut current_depth: u64) -> ProgramPatch {
        let index = find_by(commands, Some(index), |command| {
            match command {
                &Command::JumpBackward => current_depth -= 1,
                &Command::JumpForward => current_depth += 1,
                _ => {},
            };
            return current_depth == goal_depth;
        });

        return match index {
            Some(index) => ProgramPatch {
                instruction_pointer: index,
                status: ProgramStatus::Normal,
                current_depth
            },

            None => ProgramPatch {
                instruction_pointer: commands.len(),
                status: ProgramStatus::Seeking(goal_depth),
                current_depth,
            }
        };
    }
}
