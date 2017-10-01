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
            '[' => Some(Command::JumpForward),
            ']' => Some(Command::JumpBackward),
            _ => None,
        };
        match command {
            Some(com) => vec.push(com),
            None => {}
        }
    }
    vec
}


#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_parse() {
        const COMMAND_TOUR: &'static str = ".,><+-[]9";
        let actual = parse(&COMMAND_TOUR);
        let expected = [
            Command::Output,
            Command::Input,
            Command::RightShift,
            Command::LeftShift,
            Command::Increment,
            Command::Decrement,
            Command::JumpForward,
            Command::JumpBackward,
        ];
        assert_eq!(actual, expected);
    }
}
