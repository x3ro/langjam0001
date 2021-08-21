use crate::parser::parse;
use crate::evaluator::evaluate;

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

    let _simple_src = r#"
fn plus3(a) {
    plus(a, 3)
}

print(plus3(5))
    "#;

    let super_simple_src = r#"
print("foo")
    "#;

    let res = parse(super_simple_src).unwrap();

    for node in res {
        evaluate(node);
    }
}