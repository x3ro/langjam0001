use pest::Parser;
use pest::error::Error;
use crate::parser::AstNode::Assignment;

#[derive(Parser)]
#[grammar = "grammar.pest"]
pub struct Lj1Parser;

#[derive(Debug, Clone)]
pub enum AstNode {
    // Print(Box<AstNode>),
    Call {
        identifier: String,
        arguments: Vec<AstNode>,
    },

    Definition {
        identifier: String,
        arguments: Vec<String>,
        body: Vec<AstNode>,
    },

    Assignment {
        identifier: String,
        value: Box<AstNode>,
    },

    Identifier(String),
    Integer(i64),
    String(String),
    NoOp
}

pub fn parse(source: &str) -> Result<Vec<AstNode>, Error<Rule>> {
    let mut ast = vec![];

    let pairs = Lj1Parser::parse(Rule::file, source)?;
    //println!("{:#?}", pairs);

    for pair in pairs {
        match pair.as_rule() {
            Rule::expression => {
                ast.push(build_ast_from_expression(pair));
            }

            Rule::statement => {
                ast.push(build_ast_from_statement(pair))
            }

            Rule::EOI => {
                break;
            }

            x => panic!("Unknown rule '{:?}'", x)
        }
    }

    Ok(ast)
}

fn build_ast_from_statement(pair: pest::iterators::Pair<Rule>) -> AstNode {
    assert_eq!(pair.as_rule(), Rule::statement);

    let mut inner = pair.into_inner();
    // assert_eq!(inner.count(), 1);

    let pair = inner.next().unwrap();
    //println!("{:#?}", pair);

    match pair.as_rule() {
        Rule::function_definition => {
            let mut pair = pair.into_inner();
            let identifier = pair.next().unwrap().as_str();
            let argument_list = pair.next().unwrap().into_inner().map(|x| x.as_str().into()).collect();

            let mut body = vec![];
            while pair.peek().is_some() {
                let expr = pair.next().unwrap();
                body.push(build_ast_from_expression(expr));
            }

            AstNode::Definition {
                identifier: identifier.into(),
                arguments: argument_list,
                body
            }
        }

        Rule::assignment => {
            let mut pair = pair.into_inner();
            let identifier = pair.next().unwrap().as_str().into();
            let value = Box::new(build_ast_from_expression(pair.next().unwrap()));

            AstNode::Assignment {
                identifier,
                value,
            }
        }

        x => panic!("Unknown rule '{:?}' parsing {:#?}", x, pair)
    }
}

fn build_ast_from_expression(pair: pest::iterators::Pair<Rule>) -> AstNode {
    assert_eq!(pair.as_rule(), Rule::expression);
    //println!("---------\n{:#?}", pair);

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

        Rule::identifier => {
            AstNode::Identifier(pair.as_str().into())
        }

        x => panic!("Unknown rule '{:?}' parsing {:#?}", x, pair)
    }
}
