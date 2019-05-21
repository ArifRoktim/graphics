pub mod ast;
pub use ast::{AstNode, Axis, Command};
use pest_derive::*;

use lib_graphics::Shine;
use std::collections::HashMap;
use std::default::Default;
use std::fs;

type SymbolTable<'a> = HashMap<&'a str, Vec<AstNode>>;
#[derive(Debug)]
pub struct Operation {
    pub command: Command,
    pub args: Vec<AstNode>,
    pub constants: Option<[Shine; 3]>,
}
impl Operation {
    pub fn new(command: Command, args: Vec<AstNode>, constants: Option<[Shine; 3]>) -> Operation {
        Operation { command, args, constants }
    }
}

#[derive(Debug)]
pub struct ToDoList<'a> {
    pub ops: Vec<Operation>,
    pub symbols: SymbolTable<'a>,
}
impl<'a> ToDoList<'a> {
    pub fn push_op(
        &mut self,
        command: &Command,
        args: Vec<AstNode>,
        constants: Option<[Shine; 3]>,
    ) {
        self.ops.push(Operation::new(command.to_owned(), args, constants));
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

#[derive(Debug)]
pub enum ParseError {
    AstIntoError,
    ParseError,
    SemanticError,
}
impl From<ast::AstIntoError> for ParseError {
    fn from(_: ast::AstIntoError) -> ParseError {
        ParseError::AstIntoError
    }
}
impl From<ast::ParseAxisError> for ParseError {
    fn from(_: ast::ParseAxisError) -> ParseError {
        ParseError::ParseError
    }
}

impl MDLParser {
    pub fn file(filename: &str) -> Result<(), ParseError> {
        let file = fs::read_to_string(filename).expect("Error reading file!");
        let nodes = ast::parse(&file).expect("Failed while performing parsing!");
        Self::analyze_nodes(&nodes)?;
        Ok(())
    }

    fn analyze_nodes(nodes: &[AstNode]) -> Result<(), ParseError> {
        let mut todo = ToDoList::default();
        for node in nodes {
            Self::analyze(node, &mut todo)?;
        }
        dbg!(&todo);
        Ok(())
    }

    fn analyze(node: &AstNode, todo: &mut ToDoList) -> Result<(), ParseError> {
        if let AstNode::MdlCommand { command, args } = node {
            // TODO: Iterate through `args` when we eventually need to do a
            // post order traversal on the Ast
            // In which case, make the `node` argument mutable, then replace
            // each `expression` with its resulting value

            //todo.push_op(command, args);
            Ok(())
        } else {
            // TODO: Change this when the Ast becomes more complex and has expressions
            unreachable!()
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::PathBuf;

    #[allow(dead_code)]
    fn get_mdl() -> String {
        let mut mdl_file = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        mdl_file.push("tests/face.mdl");
        fs::read_to_string(&mdl_file).expect("face.mdl missing!")
    }

    #[test]
    fn test_analyze() -> Result<(), ParseError> {
        let text = "
push
pop
save foo.bar
";
        let nodes = ast::parse(&text).expect("Failed while performing parsing!");
        dbg!(&nodes);
        MDLParser::analyze_nodes(&nodes[..])?;
        Ok(())
    }

    #[test]
    fn mdl_analyze() -> Result<(), ParseError> {
        let nodes = ast::parse(&get_mdl()).unwrap();
        dbg!(&nodes);
        MDLParser::analyze_nodes(&nodes)?;
        Ok(())
    }
}
