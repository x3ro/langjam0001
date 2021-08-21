extern crate pest;
#[macro_use]
extern crate pest_derive;

use pest::Parser;
use pest::error::Error;

#[derive(Parser)]
#[grammar = "grammar.pest"]
pub struct Lj1Parser;

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
print(plus3(5, 4))
    "#;

    let res = parse(super_simple_src);
    println!("{:#?}", res);
}

#[derive(Debug)]
pub enum AstNode {
    Print(Box<AstNode>),
    Call {
        identifier: String,
        arguments: Vec<AstNode>,
    },
    NoOp
}

pub fn parse(source: &str) -> Result<Vec<AstNode>, Error<Rule>> {
    let mut ast = vec![];

    let pairs = Lj1Parser::parse(Rule::file, source)?;
    println!("{:#?}", pairs);
    for pair in pairs {
        match pair.as_rule() {
            Rule::expression => {
                ast.push(build_ast_from_expression(pair));
                // for inner_pair in pair.into_inner() {
                //     ast.push(build_ast_from_expression(inner_pair));
                // }

            }

            Rule::EOI => {
                break;
            }

            x => panic!("Unknown rule '{:?}'", x)
        }
    }

    Ok(ast)
}

fn build_ast_from_expression(pair: pest::iterators::Pair<Rule>) -> AstNode {
    assert_eq!(pair.as_rule(), Rule::expression);

    let mut inner = pair.into_inner();
    // assert_eq!(inner.count(), 1);

    let pair = inner.next().unwrap();
    match pair.as_rule() {
        Rule::function_call => {
            let mut pair = pair.into_inner();
            let identifier = pair.next().unwrap();
            let argument_list = pair.next().unwrap().into_inner();

            println!("{:#?}", argument_list);
            AstNode::Call {
                identifier: identifier.as_str().into(),
                arguments: argument_list.map(|inner| build_ast_from_expression(inner)).collect()
            }
        }
        x => panic!("Unknown rule '{:?}'", x)
    }
}


