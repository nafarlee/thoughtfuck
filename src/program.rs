use command::Command;


pub struct Program {
    instructions : Vec<Command>,
    is_seeking: bool,
    current_depth: u64,
    goal_depth: Option<u64>,
}


impl Program {
    pub fn new () -> Program {
        Program {
            instructions: Vec::new(),
            is_seeking: false,
            current_depth: 0,
            goal_depth: None,
        }
    }


    pub fn append(&mut self, instructions: &[Command]) {
        self.instructions.extend(instructions.iter().cloned());
    }
}
