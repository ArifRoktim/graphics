pub mod ast;
pub use ast::{Command, Axis, AstNode, ParseAxisError};

use pest_derive::*;
use std::fs;
use std::collections::HashMap;
use std::default::Default;

#[derive(Clone, Debug)]
pub enum Args {
    Float(f64),
    Ident(String),
    Str(String),
    Axis(Axis),
}

type SymbolTable<'a> = HashMap<&'a str, Vec<Args>>;
#[derive(Debug)]
pub struct Operation {
    pub command: Command,
    pub args: Vec<Args>,
}
impl Operation {
    pub fn new(command: Command, args: Vec<Args>) -> Operation {
        Operation { command, args }
    }
}

#[derive(Debug)]
pub struct ToDoList<'a> {
    pub ops: Vec<Operation>,
    pub symbols: SymbolTable<'a>,
}
impl<'a> ToDoList<'a> {
    pub fn push_op(&mut self, command: &Command, args: &[Args]) {
        self.ops.push(Operation::new(command.to_owned(), args.to_vec()));
    }
}
impl<'a> Default for ToDoList<'a> {
    fn default() -> Self {
        let ops = vec![];
        let symbols: SymbolTable = HashMap::new();
        ToDoList { ops, symbols }
    }
}


#[derive(Parser)]
#[grammar = "mdl.pest"]
pub struct MDLParser;

impl MDLParser {
    pub fn file(filename: &str) {
        let file = fs::read_to_string(filename).expect("Error reading file!");
        let nodes = ast::parse(&file).expect("Failed while performing parsing!");
        Self::analyze_nodes(&nodes);
    }

    fn analyze_nodes(nodes: &[AstNode]) {
        let mut todo = ToDoList::default();
        for node in nodes {
            Self::analyze(node, &mut todo);
            // Do post order traversal by checking if any of the `Expression` nodes
            // have arguments that are also `Expression`s

        }
        dbg!(&todo);
    }

    fn analyze(node: &AstNode, todo: &mut ToDoList) {
        //dbg!(&node);
        if let AstNode::Expression {command, args} = node {
            match command {
                Command::Push => todo.push_op(command, &[]),
                Command::Pop => todo.push_op(command, &[]),
                Command::Display => todo.push_op(command, &[]),
                Command::Save => {
                },
                _ => unimplemented!(),
            }
        } else {
            unreachable!()
        }
    }
}

#[cfg(test)]
mod tests {
    use std::path::PathBuf;
    use super::*;

    fn get_mdl() -> String {
        let mut mdl_file = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        mdl_file.push("tests/face.mdl");
        fs::read_to_string(&mdl_file).expect("face.mdl missing!")
    }

    #[test]
    fn test_analyze() {
        let text = "
push
pop
push
";
        //dbg!(&text);
        let nodes = ast::parse(&text).expect("Failed while performing parsing!");
        MDLParser::analyze_nodes(&nodes[..]);
    }
}
