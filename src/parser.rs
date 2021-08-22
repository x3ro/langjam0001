use pest::Parser;
use pest::error::Error;
use crate::parser::AstNode::Assignment;
use pest::iterators::Pairs;

#[derive(Parser)]
#[grammar = "grammar.pest"]
pub struct Lj1Parser;

#[derive(Debug, Clone)]
pub enum AstNode {
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

    // e.g. foo.$comment
    MetaPropertyAccess {
        lhs: String,
        rhs: String,
    },

    Integer(i64),
    String(String),
    Comment(String),
    NoOp
}

pub fn parse(source: &str) -> Result<Vec<AstNode>, Error<Rule>> {
    let mut ast = vec![];

    let pairs = Lj1Parser::parse(Rule::file, source)?;
    //println!("{:#?}", pairs);

    for pair in pairs {
        match pair.as_rule() {
            Rule::expression | Rule::statement => {
                ast.push(build_ast_from_statement_or_expression(pair));
            }

            Rule::EOI => {
                break;
            }

            x => panic!("Unknown rule '{:?}'", x)
        }
    }

    Ok(ast)
}

fn extract_comment(pairs: &mut pest::iterators::Pairs<Rule>) -> Option<AstNode> {
    if pairs.peek().unwrap().as_rule() != Rule::comment {
        None
    } else {
        Some(AstNode::Comment(pairs.next().unwrap().as_str().into()))
    }
}

fn build_ast_from_statement_or_expression(pair: pest::iterators::Pair<Rule>) -> AstNode {
    match pair.as_rule() {
        Rule::statement => build_ast_from_statement(pair),
        Rule::expression => build_ast_from_expression(pair),
        x => panic!("Unknown rule '{:?}' parsing {:#?}", x, pair)
    }
}

fn build_ast_from_statement(pair: pest::iterators::Pair<Rule>) -> AstNode {
    assert_eq!(pair.as_rule(), Rule::statement);

    let mut inner = pair.into_inner();
    // assert_eq!(inner.count(), 1);

    let comment = extract_comment(&mut inner);
    //println!("{:?}", comment);

    let pair = inner.next().unwrap();

    match pair.as_rule() {
        Rule::function_definition => {
            let mut pair = pair.into_inner();
            let identifier = pair.next().unwrap().as_str();
            let argument_list = pair.next().unwrap().into_inner().map(|x| x.as_str().into()).collect();

            let mut body = vec![];
            while pair.peek().is_some() {

                let node = pair.next().unwrap();
                body.push(build_ast_from_statement_or_expression(node));
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
    let mut inner = pair.into_inner();

    let comment = extract_comment(&mut inner);
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

        Rule::meta_property_access => {
            let mut pair = pair.into_inner();

            let lhs = pair.next().unwrap().as_str().into();
            let rhs = pair.next().unwrap().as_str().into();

            AstNode::MetaPropertyAccess {
                lhs,
                rhs
            }
        }

        x => panic!("Unknown rule '{:?}' parsing {:#?}", x, pair)
    }
}
