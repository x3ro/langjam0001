use crate::parser::parse;
use crate::evaluator::{Eval, State};

#[macro_use]
extern crate pest_derive;

mod parser;
mod evaluator;

fn main() {
    let _src = r#"
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

    let simple_src = r#"
fn plus3(a) {
    plus(a, 3)
}

print(plus3(5))
    "#;

    let _super_simple_src = r#"
print("foo")
print("bar")
print(4)
print(plus(4,4,1,2,3,4))
    "#;

    let res = parse(simple_src).unwrap();

    let mut state = State::new();
    for node in res {
        node.evaluate(&mut state);
    }
}