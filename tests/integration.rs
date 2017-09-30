extern crate thoughtfuck;

mod common;


#[test]
fn hello_world () {
    common::test_output("./tests/hello.b", "./tests/hello.txt");
}
