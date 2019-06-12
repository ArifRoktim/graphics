use super::{ObjParser, Rule};
use pest::error::Error;
use pest::iterators::Pair;
use pest::Parser;

#[derive(Clone, Debug)]
pub enum ParseCommand {
    Vertex,
    Face,
    NOOP,
}

impl From<&Rule> for ParseCommand {
    fn from(r: &Rule) -> ParseCommand {
        use ParseCommand as Pcmd;
        use Rule::*;
        match r {
            vertex => Pcmd::Vertex,
            face => Pcmd::Face,

            // ignored commands
            group => Pcmd::NOOP,

            // Statements that are handled by `node_from_statement`
            // Primitve `Rule`s aren't converted to `ParseCommand`s
            float | whole => panic!("{:?} is not a command!", r),
            // These are already unwrapped
            statement | SKIP | EOI => unreachable!(),
            // These are silent
            program | WHITESPACE | COMMENT => unreachable!(),
        }
    }
}

#[derive(Clone, Debug)]
pub enum AstNode {
    Float(f64),
    Whole(usize),
    ObjCommand { command: ParseCommand, args: Vec<AstNode> },
}

pub fn parse(source: &str) -> Result<Vec<AstNode>, Error<Rule>> {
    let mut ast: Vec<AstNode> = vec![];

    let pairs = ObjParser::parse(Rule::program, source)?;
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
        Rule::statement => {
            node_from_statement(
                // extract a match from the statement; never fails
                pair.into_inner().next().unwrap(),
            )
        },
        // Primitives
        Rule::float => AstNode::Float(pair.as_str().parse::<f64>().unwrap()),
        Rule::whole => AstNode::Whole(pair.as_str().parse::<usize>().unwrap()),
        // These are silent or already unwrapped
        Rule::EOI | Rule::program | Rule::WHITESPACE | Rule::COMMENT => unreachable!(),
        // Commands
        rule => AstNode::ObjCommand { command: ParseCommand::from(&rule), args: get_args(pair) },
    }
}

fn get_args(pair: Pair<Rule>) -> Vec<AstNode> {
    pair.into_inner().map(node_from_statement).collect()
}
