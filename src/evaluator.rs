use crate::parser::AstNode;
use std::collections::HashMap;
use std::rc::Rc;

pub struct State {
    fns: HashMap<String, Rc<dyn Eval>>,
    arguments: Vec<AstNode>,
}

impl State {
    pub fn new() -> State {
        let mut fns: HashMap<String, Rc<dyn Eval>> = HashMap::new();

        // Built-in functions
        fns.insert("print".into(), Rc::new(BuiltinPrint{}));
        fns.insert("plus".into(), Rc::new(BuiltinPlus{}));

        State {
            fns,
            arguments: vec![],
        }
    }
}

pub trait Eval {
    fn evaluate(&self, state: &mut State) -> AstNode;
}

impl Eval for AstNode {
    fn evaluate(&self, mut state: &mut State) -> AstNode {
        match self {
            // AstNode::Print(x) => {
            //     println!("{:?}", x)
            // }

            AstNode::Call {
                identifier,
                arguments,
            } => {
                for arg in arguments {
                    arg.evaluate(state);
                }

                state.arguments = arguments.iter().map(|arg| arg.evaluate(state)).collect();
                let function= state.fns.get(identifier);

                if function.is_none() {
                    panic!("Tried to call unknown function '{}'", identifier)
                }

                function.unwrap().clone().evaluate(state)
            }
            AstNode::Integer(_) => self.clone(),
            AstNode::String(_) => self.clone(),

            // AstNode::NoOp => {}

            x => panic!("Unknown node '{:?}'", x)
        }

        //println!("Running {:#?}", node);
    }
}

struct BuiltinPrint;
impl Eval for BuiltinPrint {
    fn evaluate(&self, state: &mut State) -> AstNode {
        for arg in &state.arguments {
            println!("{:?}", arg)
        }
        AstNode::NoOp
    }
}

struct BuiltinPlus;
impl Eval for BuiltinPlus {
    fn evaluate(&self, state: &mut State) -> AstNode {
        let mut sum:i64 = 0;
        for arg in &state.arguments {
            match arg {
                AstNode::Integer(x) => {
                    sum += x;
                }
                x => panic!("Tried to call plus with invalid value '{:?}'", x)
            }
        }
        AstNode::Integer(sum)
    }
}