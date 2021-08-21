use pest::Parser;
use pest::error::Error;

#[derive(Parser)]
#[grammar = "grammar.pest"]
pub struct Lj1Parser;

#[derive(Debug)]
pub enum AstNode {
    Print(Box<AstNode>),
    Call {
        identifier: String,
        arguments: Vec<AstNode>,
    },
    Integer(i64),
    String(String),
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

            AstNode::Call {
                identifier: identifier.as_str().into(),
                arguments: argument_list.map(|inner| build_ast_from_expression(inner)).collect()
            }
        }

        Rule::integer => {
            AstNode::Integer(pair.as_str().parse().unwrap())
        }

        Rule::string => {
            let s = pair.as_str();
            AstNode::String(s[1..s.len()-1].into())
        }

        x => panic!("Unknown rule '{:?}'", x)
    }
}
