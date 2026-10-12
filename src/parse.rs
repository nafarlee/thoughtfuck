use crate::command::Command;

pub fn parse(s: &str) -> Vec<Command> {
    s.chars()
        .filter_map(|c| match c {
            '.' => Some(Command::Output),
            ',' => Some(Command::Input),
            '<' => Some(Command::LeftShift),
            '>' => Some(Command::RightShift),
            '+' => Some(Command::Increment),
            '-' => Some(Command::Decrement),
            '[' => Some(Command::JumpForward),
            ']' => Some(Command::JumpBackward),
            _ => None,
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_parse() {
        let command_tour = ".,><+-[]9";
        let actual = parse(command_tour);
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
