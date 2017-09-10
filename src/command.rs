#[derive(Debug,Clone,PartialEq,Copy)]
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
