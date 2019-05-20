use parse_graphics::*;
use pest::Parser;

use std::fs;
use std::path::PathBuf;

fn get_mdl() -> String {
    let mut mdl_file = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    mdl_file.push("tests/face.mdl");
    fs::read_to_string(&mdl_file).expect("face.mdl missing!")
}

#[test]
fn test_pest() {
    let mdl = get_mdl();
    dbg!(MDLParser::parse(Rule::program, &mdl).expect("Error parsing file!"));
}

#[test]
fn test_ast() {
    let mdl = get_mdl();
    let nodes = parse(&mdl).unwrap();
    dbg!(&nodes);
}
