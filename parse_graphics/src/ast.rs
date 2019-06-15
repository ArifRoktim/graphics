use super::{MDLParser, Rule};
use pest::error::Error;
use pest::iterators::Pair;
use pest::Parser;
use std::f64;
use std::str::FromStr;

// TODO: Rename this to `ParseStatement`
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
    Light,
    Mesh,
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
            light => Pcmd::Light,
            mesh => Pcmd::Mesh,

            // The following aren't commands
            expr | add | subtract | multiply | divide | number
                // Primitve `Rule`s
                | float | integer | axis | ident | string
                // we don't parse the end of input
                | EOI
                => unreachable!("`{:?}` not a command!", r),
            // These are silent
            program | statement | term | operation | WHITESPACE | COMMENT => unreachable!(),
        }
    }
}

// TODO?: Move this enum to lib_graphics
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

#[derive(Clone, Debug)]
pub enum Operation {
    Add,
    Subtract,
    Multiply,
    Divide,
}

#[derive(Clone, Debug)]
pub enum Number {
    Float(f64),
    Integer(isize),
}

impl From<&Number> for f64 {
    fn from(num: &Number) -> f64 {
        use Number::*;
        match *num {
            Float(f) => f,
            Integer(i) => i as f64,
        }
    }
}

// TODO: Add a AstNode::new_mdl method
#[derive(Clone, Debug)]
pub enum AstNode {
    Num(Number),
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
            Rule::EOI => break,
            _ => ast.push(node_from_statement(pair)),
        }
    }
    Ok(ast)
}

fn node_from_statement(pair: Pair<Rule>) -> AstNode {
    use self::Axis as PAxis;
    use AstNode::*;
    use Number::*;

    match pair.as_rule() {
        // Primitives
        Rule::float => Num(Float(pair.as_str().parse().unwrap())),
        Rule::integer => Num(Integer(pair.as_str().parse().unwrap())),
        Rule::axis => Axis(pair.as_str().parse::<PAxis>().unwrap()),
        Rule::ident => Ident(pair.as_str().to_owned()),
        Rule::string => Str(pair.as_str().to_owned()),
        // These are silent or already unwrapped
        Rule::EOI | Rule::program | Rule::statement | Rule::WHITESPACE | Rule::COMMENT => {
            unreachable!()
        },
        // Commands
        rule => AstNode::MdlCommand { command: ParseCommand::from(&rule), args: get_args(pair) },
    }
}

fn get_args(pair: Pair<Rule>) -> Vec<AstNode> {
    pair.into_inner().map(node_from_statement).collect()
}
