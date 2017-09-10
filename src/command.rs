#[derive(Debug,Clone)]
pub enum Command {
    JumpForward,
    JumpBackward,
    Output,
    Input,
    Increment,
    Decrement,
    RightShift,
    LeftShift
}
