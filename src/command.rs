#[derive(Debug,Clone)]
pub enum Command {
    Output,
    Input,
    Increment,
    Decrement,
    RightShift,
    LeftShift
}
