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

impl From<&Rule> for ParseCommand {
    fn from(r: &Rule) -> ParseCommand {
        use ParseCommand as Pcmd;
        use Rule::*;
        match r {
            push => Pcmd::Push,
            pop => Pcmd::Pop,
            display => Pcmd::Display,
            save => Pcmd::Save,
            translate => Pcmd::Translate,
            scale => Pcmd::Scale,
            rotate => Pcmd::Rotate,
            cuboid => Pcmd::Cuboid,
            sphere => Pcmd::Sphere,
            torus => Pcmd::Torus,
            line => Pcmd::Line,
            constants => Pcmd::Constants,
            frames => Pcmd::Frames,
            basename => Pcmd::Basename,
            vary => Pcmd::Vary,

            // Statements that are handled by `node_from_statement`
            // Primitve `Rule`s aren't converted to `ParseCommand`s
            float | whole | axis | ident | string => panic!("{:?} is not a command!", r),
            // TODO: Might add expressions to language later
            statement => panic!("Parse error!"),
            // These are silent or already unwrapped
            EOI | program | WHITESPACE | COMMENT => unreachable!(),
        }
    }
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
    Whole(usize),
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
        // Primitives
        Rule::float => AstNode::Float(pair.as_str().parse::<f64>().unwrap()),
        Rule::whole => AstNode::Whole(pair.as_str().parse::<usize>().unwrap()),
        Rule::axis => AstNode::Axis(pair.as_str().parse::<Axis>().unwrap()),
        Rule::ident => AstNode::Ident(pair.as_str().to_owned()),
        Rule::string => AstNode::Str(pair.as_str().to_owned()),
        // These are silent or already unwrapped
        Rule::EOI | Rule::program | Rule::WHITESPACE | Rule::COMMENT => unreachable!(),
        // Commands
        rule => AstNode::MdlCommand { command: ParseCommand::from(&rule), args: get_args(pair) },
    }
}

fn get_args(pair: Pair<Rule>) -> Vec<AstNode> {
    pair.into_inner().map(node_from_statement).collect()
}
