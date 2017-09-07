fn main() {
    loop {
        let mut line = String::new();
        match std::io::stdin().read_line(&mut line) {
            Ok(0) => break,
            Ok(_) => {
                print!("{}", line);
            },
            Err(error) => print!("{}", error),
        }
    }
}
