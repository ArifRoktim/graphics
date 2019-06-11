use pest_derive::Parser;
#[derive(Parser)]
#[grammar = "obj.pest"]
pub struct ObjParser;
