use parse_obj::{ast, ObjParser, Rule};
use pest::error::Error;
use pest::iterators::Pairs;
use pest::Parser;

use std::fs;
use std::path::PathBuf;

#[test]
fn parse_float() {
    assert_eq!("", as_str(ObjParser::parse(Rule::float, "-NUMBER HERE 1.0")));
    assert_eq!("", as_str(ObjParser::parse(Rule::float, ".")));

    assert_eq!("1", as_str(ObjParser::parse(Rule::float, "1")));
    assert_eq!("-1", as_str(ObjParser::parse(Rule::float, "-1")));
    assert_eq!("123", as_str(ObjParser::parse(Rule::float, "123")));

    assert_eq!("1.", as_str(ObjParser::parse(Rule::float, "1.")));
    assert_eq!("-1.", as_str(ObjParser::parse(Rule::float, "-1.")));

    assert_eq!("1.2", as_str(ObjParser::parse(Rule::float, "1.2")));

    assert_eq!(".2", as_str(ObjParser::parse(Rule::float, ".2")));
    assert_eq!("-.2", as_str(ObjParser::parse(Rule::float, "-.2")));

    assert_eq!("1.2", as_str(ObjParser::parse(Rule::float, "1.2.3")));
    assert_eq!(".2", as_str(ObjParser::parse(Rule::float, ".2.3")));

    assert_eq!("45.0000", as_str(ObjParser::parse(Rule::float, "45.0000")));
    assert_eq!("0.000000E+00", as_str(ObjParser::parse(Rule::float, "0.000000E+00")));
    assert_eq!("3.5e6", as_str(ObjParser::parse(Rule::float, "3.5e6")));
    assert_eq!("3.5e+6", as_str(ObjParser::parse(Rule::float, "3.5e+6")));
    assert_eq!("3.5e-6", as_str(ObjParser::parse(Rule::float, "3.5e-6")));
}


#[test]
fn parse_vertex() {
    assert_eq!("v 1 2 3", as_str(ObjParser::parse(Rule::vertex, "v 1 2 3")));
    assert_eq!("v 0.5 -.5 -5.2", as_str(ObjParser::parse(Rule::vertex, "v 0.5 -.5 -5.2")));
}

#[test]
fn parse_face() {
    assert_eq!("f 1 2 3", as_str(ObjParser::parse(Rule::face, "f 1 2 3")));
    assert_eq!("", as_str(ObjParser::parse(Rule::face, "f 0 -.5 5.2")));
}

fn as_str(parsed: Result<Pairs<'_, Rule>, Error<Rule>>) -> &str {
    dbg!(parsed).ok().map_or("", |s| s.as_str())
}
