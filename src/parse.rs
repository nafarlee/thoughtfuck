use command::Command;

pub fn parse(s: &str) -> Vec<Command> {
    let mut vec = Vec::new();
    for c in s.chars() {
        let command = match c {
            '.' => Some(Command::Output),
            ',' => Some(Command::Input),
            '<' => Some(Command::LeftShift),
            '>' => Some(Command::RightShift),
            '+' => Some(Command::Increment),
            '-' => Some(Command::Decrement),
            _   => None,
        };
        match command {
            Some(com) => vec.push(com),
            None => {},
        }
    }
    vec
}
