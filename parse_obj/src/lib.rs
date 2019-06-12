pub mod ast;

use ast::AstNode::*;
use ast::ParseCommand as PCmd;
use lib_graphics::Matrix;
use std::error::Error;
use std::f64::NEG_INFINITY as NEG_INF;
use std::fmt;

use pest_derive::Parser;
#[derive(Parser)]
#[grammar = "obj.pest"]
pub struct ObjParser;

#[derive(Debug)]
pub struct SemanticError;
impl fmt::Display for SemanticError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Semantic Error!")
    }
}
impl Error for SemanticError {}

impl ObjParser {
    pub fn load(matrix: &mut Matrix, mesh: &str) -> Result<(), Box<dyn Error>> {
        let nodes = ast::parse(mesh)?;
        // Create a vector for the vertices.
        // The .obj format one-indexes the vertex list, so push a dummy vertex for
        // index 0
        let mut vertices = vec![[NEG_INF, NEG_INF, NEG_INF, 1.0]];
        for node in nodes {
            if let ObjCommand { command, args } = node {
                match command {
                    // add a vertex to the vertex list
                    PCmd::Vertex => {
                        if let [Float(x), Float(y), Float(z)] = args[..] {
                            vertices.push([x, y, z, 1.0]);
                        } else {
                            return Err(Box::new(SemanticError));
                        }
                    },
                    // push a triangle to the matrix
                    PCmd::Face => {
                        if let [Whole(x), Whole(y), Whole(z)] = args[..] {
                            for &vertex in &[x, y, z] {
                                let point = vertices.get(vertex).ok_or(SemanticError)?;
                                matrix.push(*point);
                            }
                        } else {
                            return Err(Box::new(SemanticError));
                        }
                    },
                    // do nothing
                    PCmd::NOOP => {},
                }
            }
        }
        Ok(())
    }
}
