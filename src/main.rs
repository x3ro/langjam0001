extern crate pest;
#[macro_use]
extern crate pest_derive;

use pest::Parser;
use pest::error::Error;

#[derive(Parser)]
#[grammar = "grammar.pest"]
pub struct Lj1Parser;

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

    let simple_src = r#"
fn plus3(a) {
    plus(a, 3)
}

print(plus3(5))
    "#;

    //let res = Lj1Parser::parse(Rule::file, simple_src);
    let res = parse(simple_src);
    println!("{:#?}", res);
}

#[derive(Debug)]
pub enum AstNode {
    Print(Box<AstNode>),
    Noop
}

pub fn parse(source: &str) -> Result<Vec<AstNode>, Error<Rule>> {
    let mut ast = vec![];

    let pairs = Lj1Parser::parse(Rule::file, source)?;
    println!("{:#?}", pairs);
    for pair in pairs {
        match pair.as_rule() {
            Rule::expression => {
                ast.push(build_ast_from_expr(pair));
            }

            Rule::EOI => {
                break;
            }
            // Rule::expr => {
            //     ast.push(Print(Box::new(build_ast_from_expr(pair))));
            // }
            x => {
                panic!("Unknown rule '{:?}'", x)
            }
        }
    }

    Ok(ast)
}

fn build_ast_from_expr(pair: pest::iterators::Pair<Rule>) -> AstNode {
    AstNode::Noop
}

