use pest::*;
use pest_derive::*;
use pest::error::Error;
use pest::iterators::Pair;

use std::str::FromStr;

#[derive(Parser)]
#[grammar = "mdl.pest"]
pub struct MDLParser;

#[derive(Debug)]
pub enum Command {
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
    Constants
}

#[derive(Debug)]
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

#[derive(Debug)]
pub enum AstNode {
    Float(f64),
    Ident(String),
    Axis(Axis),
    Expression {
        command: Command,
        args: Vec<AstNode>,
    },
}

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

    traverse_tree(&mut ast);
    Ok(ast)
}

fn node_from_statement(pair: Pair<Rule>) -> AstNode {
    match pair.as_rule() {
        // Recursion will be useful in future for more complex statements
        Rule::statement => {
            node_from_statement(
                // extract a match from the statement; never fails
                pair.into_inner().next().unwrap()
            )
        },
        // No args
        Rule::push => AstNode::Expression {
            command: Command::Push,
            args: vec![],
        },
        Rule::pop => AstNode::Expression {
            command: Command::Pop,
            args: vec![],
        },
        Rule::display => AstNode::Expression {
            command: Command::Display,
            args: vec![],
        },
        // Has args
        Rule::save => AstNode::Expression {
            command: Command::Save,
            args: get_args(pair),
        },
        // Transformations
        Rule::translate => AstNode::Expression {
            command: Command::Translate,
            args: get_args(pair),
        },
        Rule::scale => AstNode::Expression {
            command: Command::Scale,
            args: get_args(pair),
        },
        Rule::rotate => AstNode::Expression {
            command: Command::Rotate,
            args: get_args(pair),
        },
        // 3D objects
        Rule::cuboid => AstNode::Expression {
            command: Command::Cuboid,
            args: get_args(pair),
        },
        Rule::sphere => AstNode::Expression {
            command: Command::Sphere,
            args: get_args(pair),
        },
        Rule::torus => AstNode::Expression {
            command: Command::Torus,
            args: get_args(pair),
        },
        // others
        Rule::line => AstNode::Expression {
            command: Command::Line,
            args: get_args(pair),
        },
        Rule::constants => AstNode::Expression {
            command: Command::Constants,
            args: get_args(pair),
        },
        // Primitives
        Rule::float => AstNode::Float(pair.as_str().parse::<f64>().unwrap()),
        Rule::axis  => AstNode::Axis(pair.as_str().parse::<Axis>().unwrap()),
        Rule::ident => AstNode::Ident(pair.as_str().to_owned()),
        _ => unimplemented!()
    }
}

fn get_args(pair: Pair<Rule>) -> Vec<AstNode> {
    pair.into_inner().map(node_from_statement).collect()
}

fn traverse_tree(_trees: &mut Vec<AstNode>) {
    //TODO: Replace this with code that traverses the syntax trees
    // when the language reachers that level of complexity.
}
