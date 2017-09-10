use command::Command;


pub struct Program {
    instructions : Vec<Command>,
    is_seeking: bool,
    current_depth: u64,
    goal_depth: Option<u64>,
}
