mod vm;
mod command;

use std::io::Write;

fn main() {
    loop {
        print!("{}", "bf> ");
        std::io::stdout().flush().unwrap();

        let mut line = String::new();
        match std::io::stdin().read_line(&mut line) {
            Ok(0) => break,
            Ok(_) => print!("{}", line),
            Err(error) => print!("{}", error),
        }
    }
}
