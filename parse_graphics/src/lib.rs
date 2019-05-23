pub mod ast;
pub mod symtab;

pub use ast::{AstNode, Axis, Command};
pub use symtab::{Operation, ToDoList};
use pest_derive::*;
use std::fs;

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

pub fn file(filename: &str) -> Result<(), ParseError> {
    let file = fs::read_to_string(filename).expect("Error reading file!");
    let nodes = ast::parse(&file).expect("Failed while performing parsing!");
    analyze_nodes(&nodes)?;
    Ok(())
}

fn analyze_nodes(nodes: &[AstNode]) -> Result<ToDoList, ParseError> {
    let mut todo = ToDoList::default();
    for node in nodes {
        analyze(node, &mut todo)?;
    }
    //dbg!(&todo);
    Ok(todo)
}

fn analyze(node: &AstNode, todo: &mut ToDoList) -> Result<(), ParseError> {
    use Command::*;
    if let AstNode::MdlCommand { command, args } = node {
        // TODO: Iterate through `args` when we eventually need to do a
        // post order traversal on the Ast
        // In which case, make the `node` argument mutable, then replace
        // each `expression` with its resulting value
        match command {
            Push | Pop | Display => todo.push_op(command, vec![], None),
            Save => todo.push_op(command, args.to_vec(), None),
            // FIXME: Can take a knob
            Translate | Scale => {
                let args = args[..3].to_vec();
                todo.push_op(command, args, None);
                //dbg!(&args);
            }
            _ => unimplemented!(),
        }


        Ok(())
    } else {
        // TODO: Change this when the Ast becomes more complex and has expressions
        unreachable!()
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
move 2 5 1
";
        let nodes = ast::parse(&text).expect("Failed while performing parsing!");
        //dbg!(&nodes);
        let todo = analyze_nodes(&nodes)?;
        dbg!(&todo);
        Ok(())
    }

    #[test]
    fn mdl_analyze() -> Result<(), ParseError> {
        let nodes = ast::parse(&get_mdl()).unwrap();
        dbg!(&nodes);
        let todo = analyze_nodes(&nodes)?;
        dbg!(&todo);
        Ok(())
    }
}
