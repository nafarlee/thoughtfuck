extern crate thoughtfuck;

mod common;


#[test]
fn hello_world() {
    common::test_output("./tests/hello.b", "./tests/hello.txt");
}


#[test]
fn sierpinski() {
    common::test_output("./tests/sierpinski.b", "./tests/sierpinski.txt");
}


#[test]
fn print_brainfuck() {
    common::test_output("./tests/print_brainfuck.b", "./tests/print_brainfuck.txt");
}
