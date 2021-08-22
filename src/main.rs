use crate::parser::parse;
use crate::evaluator::{Eval, State};
use std::{fs, env};

#[macro_use]
extern crate pest_derive;

mod parser;
mod evaluator;

fn main() {
    let src = r#"
<foobar>
fn plus(
    <test> a,
    <asd> b
) {
    let x = 42
    let y = 5
    plus(4, length(a.$comment))
}

plus(b,4)
print(b.$comment)
    "#;

    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        panic!("Please specify the source file to be evaluated as the only parameter")
    }

    let contents = fs::read_to_string(args.get(1).unwrap())
        .expect("Something went wrong reading the source file");

    let res = parse(contents.as_str()).unwrap();

    let mut state = State::new();
    for node in res {
        node.evaluate(&mut state);
    }
}