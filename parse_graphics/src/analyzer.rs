use lib_graphics::{Color, Light, Reflection, Shine, Vector};
use std::convert::TryInto;
use std::error::Error;
use std::fmt::{self, Debug, Display};
use std::fs;
use std::num::TryFromIntError;

use super::ast::{
    self, AstIntoError, AstNode, Axis, Expression, Number, ParseAxisError, ParseCommand
};
use super::todo::{Symbol, ToDoList, eval_usize};

#[derive(Clone, Debug)]
pub enum Command {
    Push(),
    Pop(),
    Display(),
    Save(String),
    Translate(Expression, Expression, Expression),
    Scale(Expression, Expression, Expression),
    Rotate(Axis, Expression),
    Cuboid(Expression, Expression, Expression, Expression, Expression, Expression),
    Sphere(Expression, Expression, Expression, Expression),
    Torus(Expression, Expression, Expression, Expression, Expression),
    Line(Expression, Expression, Expression, Expression, Expression, Expression),
    Frames(Expression),
    Basename(String),
    Vary(String, Expression, Expression, Expression, Expression),
    Mesh(String),
    Constants(NOOP),
    Light(NOOP),
}

#[derive(Clone, Debug)]
pub struct NOOP;

#[derive(Debug)]
pub enum ParseError {
    AstIntoError,
    ParseError,
    SemanticError(String),
}
impl ParseError {
    pub fn sem_error<T: Debug>(data: &T) -> ParseError {
        // TODO FIXME: Fix this UGLY HIDEOUS HACK
        ParseError::SemanticError(format!("{:#?}", data).replace("\n", "NEWLINE"))
    }
}
impl Display for ParseError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}
impl Error for ParseError {}
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
impl From<TryFromIntError> for ParseError {
    fn from(err: TryFromIntError) -> ParseError {
        ParseError::SemanticError(format!("{:?}", err))
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
    Ok(todo)
}

#[allow(clippy::many_single_char_names)]
fn analyze(node: &AstNode, todo: &mut ToDoList) -> Result<(), ParseError> {
    use AstNode::*;
    use Command as Cmd;
    use Number::*;
    use ParseCommand as PCmd;
    use ParseError as PErr;
    if let AstNode::MdlCommand { command, args } = node {
        // TODO: Iterate through `args` when we eventually need to do a
        // post order traversal on the Ast
        // In which case, make the `node` argument mutable, then replace
        // each `expression` with its resulting value
        match command {
            PCmd::Push => todo.push_op(Cmd::Push(), None, None),
            PCmd::Pop => todo.push_op(Cmd::Pop(), None, None),
            PCmd::Display => todo.push_op(Cmd::Display(), None, None),
            PCmd::Save => {
                if let Str(file) = &args[0] {
                    todo.push_op(Cmd::Save(file.to_owned()), None, None)
                } else {
                    Err(PErr::sem_error(&node))
                }
            },

            PCmd::Translate | PCmd::Scale => {
                let mut terms: Vec<Expression> = args[..3].iter().map(|val| match val {
                    Num(i) => Ok(i.into()),
                    Expr(i) => Ok(i.clone()),
                    Ident(s) => Ok(s.into()),
                    _ => Err(PErr::sem_error(val)),
                }).collect::<Result<_, _>>()?;
                let z = terms.pop().unwrap();
                let y = terms.pop().unwrap();
                let x = terms.pop().unwrap();
                // From an Option<AstNode>:
                // If there is no 3rd arg, return Ok(None).
                // Otherwise, if the arg is an AstNode::Ident, return the inner
                //     string as an Ok(Option<String>),
                // Else, the arg isn't an AstNode::Ident, so return an Err
                let knob = args
                    .get(3)
                    .map(|s| match s {
                        Ident(s) => Ok(s.to_owned()),
                        _ => Err(PErr::sem_error(&node)),
                    })
                    .transpose()?;

                if let PCmd::Translate = command {
                    todo.push_op(Cmd::Translate(x, y, z), None, knob)
                } else {
                    todo.push_op(Cmd::Scale(x, y, z), None, knob)
                }
            },

            PCmd::Rotate => {
                let axis = if let Axis(axis) = &args[0] {
                    Ok(*axis)
                } else {
                    Err(PErr::sem_error(&node))
                }?;
                let degrees = match &args[1] {
                    Num(degrees) => Ok(degrees.into()),
                    Expr(degrees) => Ok(degrees.clone()),
                    Ident(degrees) => Ok(degrees.into()),
                    _ => Err(PErr::sem_error(&node)),
                }?;
                // From an Option<AstNode>:
                // If there is no 3rd arg, return Ok(None).
                // Otherwise, if the arg is an AstNode::Ident, return the inner
                //     string as an Ok(Option<String>),
                // Else, the arg isn't an AstNode::Ident, so return an Err
                let knob = args
                    .get(2)
                    .map(|s| match s {
                        Ident(s) => Ok(s.to_owned()),
                        _ => Err(PErr::sem_error(&node)),
                    })
                    .transpose()?;

                todo.push_op(Cmd::Rotate(axis, degrees), None, knob)
            },

            PCmd::Cuboid => {
                // First argument can be either a lighting constant or a term
                let (lighting, start) = if let Ident(light) = &args[0] {
                    (Some(light.to_owned()), 1)
                } else {
                    (None, 0)
                };
                let end = start + 6;
                let mut terms: Vec<Expression> = args[start..end].iter().map(|val| match val {
                    Num(i) => Ok(i.into()),
                    Expr(i) => Ok(i.clone()),
                    Ident(i) => Ok(i.into()),
                    _ => Err(PErr::sem_error(val)),
                }).collect::<Result<_, _>>()?;
                let d = terms.pop().unwrap();
                let w = terms.pop().unwrap();
                let h = terms.pop().unwrap();
                let z = terms.pop().unwrap();
                let y = terms.pop().unwrap();
                let x = terms.pop().unwrap();
                todo.push_op(Cmd::Cuboid(x, y, z, h, w, d), lighting, None)
            },

            PCmd::Sphere => {
                // First argument can be either a lighting constant or a term
                let (lighting, start) = if let Ident(light) = &args[0] {
                    (Some(light.to_owned()), 1)
                } else {
                    (None, 0)
                };
                let end = start + 4;
                let mut terms: Vec<Expression> = args[start..end].iter().map(|val| match val {
                    Num(i) => Ok(i.into()),
                    Expr(i) => Ok(i.clone()),
                    Ident(i) => Ok(i.into()),
                    _ => Err(PErr::sem_error(val)),
                }).collect::<Result<_, _>>()?;
                let r = terms.pop().unwrap();
                let z = terms.pop().unwrap();
                let y = terms.pop().unwrap();
                let x = terms.pop().unwrap();
                todo.push_op(Cmd::Sphere(x, y, z, r), lighting, None)
            },

            PCmd::Torus => {
                // First argument can be either a lighting constant or a float
                let (lighting, start) = if let Ident(light) = &args[0] {
                    (Some(light.to_owned()), 1)
                } else {
                    (None, 0)
                };
                let end = start + 5;
                let mut terms: Vec<Expression> = args[start..end].iter().map(|val| match val {
                    Num(i) => Ok(i.into()),
                    Expr(i) => Ok(i.clone()),
                    Ident(i) => Ok(i.into()),
                    _ => Err(PErr::sem_error(val)),
                }).collect::<Result<_, _>>()?;
                let r1 = terms.pop().unwrap();
                let r0 = terms.pop().unwrap();
                let z = terms.pop().unwrap();
                let y = terms.pop().unwrap();
                let x = terms.pop().unwrap();
                todo.push_op(Cmd::Torus(x, y, z, r0, r1), lighting, None)
            },

            PCmd::Line => {
                let mut terms: Vec<Expression> = args[..6].iter().map(|val| match val {
                    Num(i) => Ok(i.into()),
                    Expr(i) => Ok(i.clone()),
                    Ident(i) => Ok(i.into()),
                    _ => Err(PErr::sem_error(val)),
                }).collect::<Result<_, _>>()?;
                let z1 = terms.pop().unwrap();
                let y1 = terms.pop().unwrap();
                let x1 = terms.pop().unwrap();
                let z0 = terms.pop().unwrap();
                let y0 = terms.pop().unwrap();
                let x0 = terms.pop().unwrap();
                todo.push_op(Cmd::Line(x0, y0, z0, x1, y1, z1), None, None)
            },

            PCmd::Constants => {
                let name = match &args[0] {
                    Ident(name) => Ok(name.to_owned()),
                    _ => Err(PErr::sem_error(&node)),
                }?;
                // Ambient (a[rgb]), diffuse (d[rgb]) and specular (s[rgb])
                // lighting constants
                let (ar, dr, sr, ag, dg, sg, ab, db, sb) = match &args[1..10] {
                    [Num(ar), Num(dr), Num(sr), Num(ag), Num(dg), Num(sg), Num(ab), Num(db), Num(sb)] => {
                        Ok((
                            ar.into(),
                            dr.into(),
                            sr.into(),
                            ag.into(),
                            dg.into(),
                            sg.into(),
                            ab.into(),
                            db.into(),
                            sb.into(),
                        ))
                    },
                    _ => Err(PErr::sem_error(&node)),
                }?;
                let reflection = Reflection {
                    ambient: Shine::new(ar, ag, ab),
                    diffuse: Shine::new(dr, dg, db),
                    specular: Shine::new(sr, sg, sb),
                };
                let lighting = Symbol::Constant(reflection);
                todo.add_sym(name, lighting);
                Ok(())
            },

            // TODO: Push these operations into their own operations list
            // TODO: to prevent having to traverse the op list more than once
            // Animation
            PCmd::Frames => {
                let frames = match &args[0] {
                    Num(i) => Ok(i.into()),
                    Expr(i) => Ok(i.clone()),
                    // Don't allow expressions for the frame command
                    //Ident(i) => Ok(i.into()),
                    _ => Err(PErr::sem_error(&node))
                }?;
                todo.add_sym("FRAMES".into(), Symbol::Num(Number::PosInt(eval_usize(&frames, None))));
                todo.push_op(Cmd::Frames(frames), None, None)
            },

            PCmd::Basename => {
                if let Str(s) = &args[0] {
                    todo.push_op(Cmd::Basename(s.to_owned()), None, None)
                } else {
                    Err(PErr::sem_error(&node))
                }
            },

            PCmd::Vary => {
                let knob = if let Ident(knob) = &args[0] {
                    Ok(knob.to_owned())
                } else {
                    Err(PErr::sem_error(&node))
                }?;
                let mut terms: Vec<Expression> = args[1..5].iter().map(|val| match val {
                    Num(i) => Ok(i.into()),
                    Expr(i) => Ok(i.clone()),
                    Ident(i) => Ok(i.into()),
                    _ => Err(PErr::sem_error(val)),
                }).collect::<Result<_, _>>()?;
                let val1 = terms.pop().unwrap();
                let val0 = terms.pop().unwrap();
                let frame1 = terms.pop().unwrap();
                let frame0 = terms.pop().unwrap();
                todo.push_op(Cmd::Vary(knob, frame0, frame1, val0, val1), None, None)
            },

            PCmd::Light => {
                let (r, g, b) = match args[..3] {
                    [Num(PosInt(r)), Num(PosInt(g)), Num(PosInt(b))] => {
                        Ok((r.try_into()?, g.try_into()?, b.try_into()?))
                    },
                    _ => Err(PErr::sem_error(&node)),
                }?;
                let (x, y, z) = match &args[3..] {
                    [Num(x), Num(y), Num(z)] => Ok((x.into(), y.into(), z.into())),
                    _ => Err(PErr::sem_error(&node)),
                }?;
                let light = Light::new(Vector::new(x, y, z), Color::new(r, g, b));
                todo.add_light(light)
            },

            PCmd::Mesh => {
                let (lighting, mesh) = if let Ident(light) = &args[0] {
                    (Some(light.to_owned()), 1)
                } else {
                    (None, 0)
                };
                let meshfile = if let Str(meshfile) = &args[mesh] {
                    Ok(meshfile.to_owned())
                } else {
                    Err(PErr::sem_error(&node))
                }?;
                todo.push_op(Cmd::Mesh(meshfile), lighting, None)
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

    fn get_mdl() -> String {
        let mut mdl_file = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        mdl_file.push("tests/debug.mdl");
        fs::read_to_string(&mdl_file).expect("debug.mdl missing!")
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
rotate z -12.
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
