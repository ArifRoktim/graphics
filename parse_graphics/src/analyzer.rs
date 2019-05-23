use lib_graphics::Shine;
use std::fs;

use super::ast;
use super::{AstIntoError, AstNode, ParseAxisError, ParseCommand};
use super::{Command, Symbol, ToDoList};

#[derive(Debug)]
pub enum ParseError {
    AstIntoError,
    ParseError,
    SemanticError,
}
impl From<AstIntoError> for ParseError {
    fn from(_: AstIntoError) -> ParseError {
        ParseError::AstIntoError
    }
}
impl From<ParseAxisError> for ParseError {
    fn from(_: ParseAxisError) -> ParseError {
        ParseError::ParseError
    }
}

pub fn file(filename: &str) -> Result<ToDoList, ParseError> {
    let file = fs::read_to_string(filename).expect("Error reading file!");
    let nodes = ast::parse(&file).expect("Failed while performing parsing!");
    analyze_nodes(&nodes)
}

fn analyze_nodes(nodes: &[AstNode]) -> Result<ToDoList, ParseError> {
    let mut todo = ToDoList::default();
    for node in nodes {
        analyze(node, &mut todo)?;
    }
    //dbg!(&todo);
    Ok(todo)
}

#[allow(clippy::many_single_char_names)]
fn analyze(node: &AstNode, todo: &mut ToDoList) -> Result<(), ParseError> {
    use AstNode::*;
    use Command as Cmd;
    use ParseCommand as PCmd;
    if let AstNode::MdlCommand { command, args } = node {
        // TODO: Iterate through `args` when we eventually need to do a
        // post order traversal on the Ast
        // In which case, make the `node` argument mutable, then replace
        // each `expression` with its resulting value
        match command {
            PCmd::Push => todo.push_op(Cmd::Push(), None),
            PCmd::Pop => todo.push_op(Cmd::Pop(), None),
            PCmd::Display => todo.push_op(Cmd::Display(), None),
            PCmd::Save => {
                if let Str(file) = &args[0] {
                    todo.push_op(Cmd::Save(file.to_owned()), None)
                } else {
                    Err(ParseError::SemanticError)
                }
            },

            //// FIXME: Can take a knob
            PCmd::Translate | PCmd::Scale => {
                let (x, y, z) = match args[..3] {
                    [Float(x), Float(y), Float(z)] => Ok((x, y, z)),
                    _ => Err(ParseError::SemanticError),
                }?;
                if let PCmd::Translate = command {
                    todo.push_op(Cmd::Translate(x, y, z), None)
                } else {
                    todo.push_op(Cmd::Scale(x, y, z), None)
                }
            },

            PCmd::Rotate => {
                let (axis, degrees) = match args[..2] {
                    [Axis(axis), Float(degrees)] => Ok((axis, degrees)),
                    _ => Err(ParseError::SemanticError),
                }?;
                todo.push_op(Cmd::Rotate(axis, degrees), None)
            },

            PCmd::Cuboid => {
                // First argument can be either a lighting constant or a float
                let (lighting, start) = if let Ident(light) = &args[0] {
                    (Some(light.to_owned()), 1)
                } else {
                    (None, 0)
                };
                let end = start + 6;
                let (x, y, z, h, w, d) = match args[start..end] {
                    [Float(x), Float(y), Float(z), Float(h), Float(w), Float(d)] => {
                        Ok((x, y, z, h, w, d))
                    },
                    _ => Err(ParseError::SemanticError),
                }?;
                todo.push_op(Cmd::Cuboid(x, y, z, h, w, d), lighting)
            },

            PCmd::Sphere => {
                // First argument can be either a lighting constant or a float
                let (lighting, start) = if let Ident(light) = &args[0] {
                    (Some(light.to_owned()), 1)
                } else {
                    (None, 0)
                };
                let end = start + 4;
                let (x, y, z, r) = match args[start..end] {
                    [Float(x), Float(y), Float(z), Float(r)] => Ok((x, y, z, r)),
                    _ => Err(ParseError::SemanticError),
                }?;
                todo.push_op(Cmd::Sphere(x, y, z, r), lighting)
            },

            PCmd::Torus => {
                // First argument can be either a lighting constant or a float
                let (lighting, start) = if let Ident(light) = &args[0] {
                    (Some(light.to_owned()), 1)
                } else {
                    (None, 0)
                };
                let end = start + 5;
                let (x, y, z, r0, r1) = match args[start..end] {
                    [Float(x), Float(y), Float(z), Float(r0), Float(r1)] => Ok((x, y, z, r0, r1)),
                    _ => Err(ParseError::SemanticError),
                }?;
                todo.push_op(Cmd::Torus(x, y, z, r0, r1), lighting)
            },

            PCmd::Line => {
                let (x0, y0, z0, x1, y1, z1) = match args[..6] {
                    [Float(x0), Float(y0), Float(z0), Float(x1), Float(y1), Float(z1)] => {
                        Ok((x0, y0, z0, x1, y1, z1))
                    },
                    _ => Err(ParseError::SemanticError),
                }?;
                todo.push_op(Cmd::Line(x0, y0, z0, x1, y1, z1), None)
            },

            PCmd::Constants => {
                let name = match &args[0] {
                    Ident(name) => Ok(name.to_owned()),
                    _ => Err(ParseError::SemanticError),
                }?;
                // Ambient (a[rgb]), diffuse (d[rgb]) and specular (s[rgb])
                // lighting constants
                let (ar, dr, sr, ag, dg, sg, ab, db, sb) = match args[1..10] {
                    [Float(ar), Float(dr), Float(sr), Float(ag), Float(dg), Float(sg), Float(ab), Float(db), Float(sb)] => {
                        Ok((ar, dr, sr, ag, dg, sg, ab, db, sb))
                    },
                    _ => Err(ParseError::SemanticError),
                }?;
                let ambient = Shine::new(ar, ag, ab);
                let diffuse = Shine::new(dr, dg, db);
                let specular = Shine::new(sr, sg, sb);
                let lighting = Symbol::Constant(ambient, diffuse, specular);
                todo.add_sym(name, lighting)
            },
        }
    } else {
        // TODO: Change this when the Ast becomes more complex and has expressions
        unreachable!()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::PathBuf;

    #[allow(dead_code)]
    fn get_mdl() -> String {
        let mut mdl_file = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        mdl_file.push("tests/face.mdl");
        fs::read_to_string(&mdl_file).expect("face.mdl missing!")
    }

    #[test]
    fn test_analyze() -> Result<(), ParseError> {
        let text = "
push
pop
save foo.bar
move 2 5 1
scale 1 2 3
rotate x 20
rotate z -12
box 1 2 3 4 5 6
box foobar 1 2 3 4 5 6
";
        let nodes = ast::parse(&text).expect("Failed while performing parsing!");
        //dbg!(&nodes);
        let todo = analyze_nodes(&nodes)?;
        dbg!(&todo);
        Ok(())
    }

    #[test]
    fn mdl_analyze() -> Result<(), ParseError> {
        let nodes = ast::parse(&get_mdl()).unwrap();
        //dbg!(&nodes);
        let todo = analyze_nodes(&nodes)?;
        dbg!(&todo);
        Ok(())
    }
}
