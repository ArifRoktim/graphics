use lazy_static::lazy_static;
use super::{MDLParser, Rule};
use pest::error::Error;
use pest::iterators::{Pair, Pairs};
use pest::Parser;
use pest::prec_climber::*;
use std::f64;
use std::str::FromStr;
use std::convert::{TryFrom, TryInto};

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
                | float | posint | negint | axis | ident | string
                // These are silent
                | program | statement | term | operation | WHITESPACE | COMMENT 
                // we don't parse the end of input
                | EOI
                => unreachable!("`{:?}` not a command!", r),
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
    Int(isize),
    PosInt(usize),
}
impl<'i> TryFrom<Pair<'i, Rule>> for Number {
    type Error = AstIntoError;
    fn try_from(pair: Pair<Rule>) -> Result<Self, Self::Error> {
        use Number::*;
        match pair.as_rule() {
            // If the parser claims that a token is a float/integer,
            // then str::parse will always succeed
            Rule::float => Ok(Float(pair.as_str().parse().unwrap())),
            Rule::posint => Ok(PosInt(pair.as_str().parse().unwrap())),
            Rule::negint => Ok(Int(pair.as_str().parse().unwrap())),
            _ => Err(AstIntoError)
        }
    }
}
impl From<&Number> for f64 {
    fn from(num: &Number) -> f64 {
        use Number::*;
        match *num {
            Float(f) => f,
            PosInt(i) => i as f64,
            Int(i) => i as f64,
        }
    }
}

#[derive(Clone, Debug)]
pub enum Expression {
    Num(Number),
    Action(Box<Expression>, Operation, Box<Expression>),
}
impl From<&Number> for Expression {
    fn from(num: &Number) -> Expression {
        Expression::Num(num.clone())
    }
}
impl From<Number> for Expression {
    fn from(num: Number) -> Expression {
        Expression::Num(num)
    }
}
impl From<isize> for Expression {
    fn from(num: isize) -> Expression {
        Expression::Num(Number::Int(num))
    }
}
impl From<usize> for Expression {
    fn from(num: usize) -> Expression {
        Expression::Num(Number::PosInt(num))
    }
}

// TODO: Add a AstNode::new_mdl method
#[derive(Clone, Debug)]
pub enum AstNode {
    Num(Number),
    Ident(String),
    Str(String),
    Axis(Axis),
    Expr(Expression),
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

lazy_static! {
    static ref PREC_CLIMBER: PrecClimber<Rule> = {
        use Rule::*;
        use Assoc::*;

        PrecClimber::new(vec![
            Operator::new(add, Left) | Operator::new(subtract, Left),
            Operator::new(multiply, Left) | Operator::new(divide, Left),
        ])
    };
}

fn eval_expr(expr: Pairs<Rule>) -> Expression {
    use Expression::*;
    use Operation as Op;
    PREC_CLIMBER.climb(
        expr,
        |pair: Pair<Rule>| match pair.as_rule() {
            Rule::float | Rule::negint | Rule::posint => {
                Num(pair.try_into().unwrap())
            },
            Rule::expr => eval_expr(pair.into_inner()),
            _ => unreachable!("{:?}", pair),
        },
        |lhs: Expression, op: Pair<Rule>, rhs: Expression| match op.as_rule() {
            Rule::add => Action(Box::new(lhs), Op::Add, Box::new(rhs)),
            Rule::subtract => Action(Box::new(lhs), Op::Subtract, Box::new(rhs)),
            Rule::multiply => Action(Box::new(lhs), Op::Multiply, Box::new(rhs)),
            Rule::divide => Action(Box::new(lhs), Op::Divide, Box::new(rhs)),
            _ => unimplemented!()
        }
    )
}

fn node_from_statement(pair: Pair<Rule>) -> AstNode {
    use self::Axis as PAxis;
    use AstNode::*;

    match pair.as_rule() {
        // Primitives
        Rule::float | Rule::posint | Rule::negint => Num(pair.try_into().unwrap()),
        Rule::axis => Axis(pair.as_str().parse::<PAxis>().unwrap()),
        Rule::ident => Ident(pair.as_str().to_owned()),
        Rule::string => Str(pair.as_str().to_owned()),
        // These are silent or already unwrapped
        Rule::EOI | Rule::program | Rule::statement | Rule::WHITESPACE | Rule::COMMENT => {
            unreachable!("`{:?}` can't be turned into a AstNode!", pair)
        },
        // Recursion! =D
        Rule::term => {
            // terms can only have 1 pair in them, either a `number` or an `expr`
            // therefore this unwrap will never panic
            get_args(pair).pop().unwrap_or_else(|| unreachable!())
        },
        Rule::expr => Expr(eval_expr(pair.into_inner())),
        // Commands
        rule => AstNode::MdlCommand { command: ParseCommand::from(&rule), args: get_args(pair) },
    }
}

fn get_args(pair: Pair<Rule>) -> Vec<AstNode> {
    pair.into_inner().map(node_from_statement).collect()
}
