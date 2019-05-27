use pest::error::Error;
use pest::iterators::Pair;
use pest::Parser;
use std::str::FromStr;

use super::{MDLParser, Rule};

#[derive(Clone, Debug)]
pub enum ParseCommand {
    Push,
    Pop,
    Display,
    Save,
    Translate,
    Scale,
    Rotate,
    Cuboid,
    Sphere,
    Torus,
    Line,
    Constants,
    Frames,
    Basename,
    Vary,
}

// TODO: Move this enum to lib_graphics
#[derive(Clone, Copy, Debug)]
pub enum Axis {
    X,
    Y,
    Z,
}

#[derive(Debug)]
pub struct ParseAxisError;

impl FromStr for Axis {
    type Err = ParseAxisError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "x" | "X" => Ok(Axis::X),
            "y" | "Y" => Ok(Axis::Y),
            "z" | "Z" => Ok(Axis::Z),
            _ => Err(ParseAxisError),
        }
    }
}

// TODO: Add a AstNode::new_mdl method
#[derive(Clone, Debug)]
pub enum AstNode {
    Float(f64),
    Whole(u32),
    Ident(String),
    Str(String),
    Axis(Axis),
    MdlCommand { command: ParseCommand, args: Vec<AstNode> },
}
#[derive(Debug)]
pub struct AstIntoError;

pub fn parse(source: &str) -> Result<Vec<AstNode>, Error<Rule>> {
    let mut ast: Vec<AstNode> = vec![];

    let pairs = MDLParser::parse(Rule::program, source)?;
    for pair in pairs {
        match pair.as_rule() {
            Rule::statement => {
                ast.push(node_from_statement(pair));
            },
            Rule::EOI => break,
            _ => unreachable!(),
        }
    }

    Ok(ast)
}

fn node_from_statement(pair: Pair<Rule>) -> AstNode {
    match pair.as_rule() {
        // Recursion will be useful in future for more complex statements
        Rule::statement => {
            node_from_statement(
                // extract a match from the statement; never fails
                pair.into_inner().next().unwrap(),
            )
        },
        // No args
        Rule::push => AstNode::MdlCommand { command: ParseCommand::Push, args: vec![] },
        Rule::pop => AstNode::MdlCommand { command: ParseCommand::Pop, args: vec![] },
        Rule::display => AstNode::MdlCommand { command: ParseCommand::Display, args: vec![] },
        // Has args
        Rule::save => AstNode::MdlCommand { command: ParseCommand::Save, args: get_args(pair) },
        // Transformations
        Rule::translate => {
            AstNode::MdlCommand { command: ParseCommand::Translate, args: get_args(pair) }
        },
        Rule::scale => AstNode::MdlCommand { command: ParseCommand::Scale, args: get_args(pair) },
        Rule::rotate => AstNode::MdlCommand { command: ParseCommand::Rotate, args: get_args(pair) },
        // 3D objects
        Rule::cuboid => AstNode::MdlCommand { command: ParseCommand::Cuboid, args: get_args(pair) },
        Rule::sphere => AstNode::MdlCommand { command: ParseCommand::Sphere, args: get_args(pair) },
        Rule::torus => AstNode::MdlCommand { command: ParseCommand::Torus, args: get_args(pair) },
        // others
        Rule::line => AstNode::MdlCommand { command: ParseCommand::Line, args: get_args(pair) },
        Rule::constants => {
            AstNode::MdlCommand { command: ParseCommand::Constants, args: get_args(pair) }
        },
        // Animation
        Rule::frames => AstNode::MdlCommand { command: ParseCommand::Frames, args: get_args(pair) },
        Rule::basename => {
            AstNode::MdlCommand { command: ParseCommand::Basename, args: get_args(pair) }
        },
        Rule::vary => AstNode::MdlCommand { command: ParseCommand::Vary, args: get_args(pair) },
        // Primitives
        Rule::float => AstNode::Float(pair.as_str().parse::<f64>().unwrap()),
        Rule::whole => AstNode::Whole(pair.as_str().parse::<u32>().unwrap()),
        Rule::axis => AstNode::Axis(pair.as_str().parse::<Axis>().unwrap()),
        Rule::ident => AstNode::Ident(pair.as_str().to_owned()),
        Rule::string => AstNode::Str(pair.as_str().to_owned()),
        _ => unimplemented!(),
    }
}

fn get_args(pair: Pair<Rule>) -> Vec<AstNode> {
    pair.into_inner().map(node_from_statement).collect()
}
