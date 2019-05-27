use super::{Axis, ParseError};
use lib_graphics::{draw, matrix::MatrixMult, Matrix, Reflection, Screen, SquareMatrix};
use lib_graphics::{LINE_COLOR, STEPS_3D};
use std::collections::HashMap;

#[derive(Debug)]
pub enum Symbol {
    Constant(Reflection),
    Knob(f64),
    //Light, etc..
}

#[derive(Clone, Debug)]
pub struct NOOP;

#[derive(Clone, Debug)]
pub enum Command {
    Push(),
    Pop(),
    Display(),
    Save(String),
    Translate(f64, f64, f64),
    Scale(f64, f64, f64),
    Rotate(Axis, f64),
    Cuboid(f64, f64, f64, f64, f64, f64),
    Sphere(f64, f64, f64, f64),
    Torus(f64, f64, f64, f64, f64),
    Line(f64, f64, f64, f64, f64, f64),
    Constants(NOOP),
    Frames(u32),
    Basename(String),
    Vary(String, u32, u32, f64, f64),
}

type SymbolTable = HashMap<String, Symbol>;
#[derive(Debug)]
pub struct Operation {
    pub command: Command,
    pub light_const: Option<String>,
    pub knob: Option<String>,
}
impl Operation {
    pub fn new(command: Command, light_const: Option<String>, knob: Option<String>) -> Operation {
        Operation { command, light_const, knob }
    }
}

#[derive(Debug)]
pub struct ToDoList {
    pub ops: Vec<Operation>,
    pub symbols: SymbolTable,
}
impl ToDoList {
    pub fn push_op(
        &mut self,
        command: Command,
        light_const: Option<String>,
        knob: Option<String>,
    ) -> Result<(), ParseError> {
        let op = Operation::new(command, light_const, knob);
        self.ops.push(op);
        Ok(())
    }

    pub fn add_sym(&mut self, k: String, v: Symbol) -> Result<(), ParseError> {
        self.symbols.insert(k, v);
        Ok(())
    }

    fn first_pass(&self) -> Option<(String, u32)> {
        use Command::*;
        let (mut basename, mut frames) = (None, None);
        let mut vary = false;
        for operation in &self.ops {
            match &operation.command {
                Basename(s) => basename = Some(s.to_owned()),
                Frames(n) => frames = Some(*n),
                Vary(..) => vary = true,
                _ => {},
            }
        }
        // If `vary` wasn't found, we're not animating
        if ! vary {
            None
        }
        // If `vary` was found but `frames` wasn't, user error
        else if frames.is_none() {
            // TODO: Move this check to semantic analyzer
            panic!("`frames` command must also be given if `vary` command given!")
        } else {
            // Animation is a go. Set a default basename if applicable
            let base = "gif";
            let basename = basename.unwrap_or_else(|| {
                println!("`basename` command not found. Using {} as a default", base);
                String::from(base)
            } );
            Some((basename, frames.unwrap()))
        }
    }

    #[allow(clippy::many_single_char_names)]
    pub fn run(self, screen: &mut Screen, cstack: &mut Vec<SquareMatrix>) {
        use Command::*;
        //dbg!(&self);
        // Temporary edge/polygon matrix
        let mut temp = Matrix::new(0);


        for operation in &self.ops {
            //dbg!(&operation);
            // clear matrix for every operation
            temp.clear();
            let command = &operation.command;
            // From an Option<String>, get the symbol with that name from the hashmap,
            // and extract the reflection from the Constant
            let light_const = operation.light_const
                .as_ref()
                .and_then(|s| self.symbols.get(s))
                .map(|s|
                     match s {
                         Symbol::Constant(r) => r,
                         _ => unreachable!(),
                     }
                );

            match command {
                Push() => {
                    // push a copy of the last item
                    let copy = cstack.last().unwrap_or_default().clone();
                    cstack.push(copy);
                },

                Pop() => {
                    cstack.pop();
                    // Make sure that the stack is never empty
                    if cstack.is_empty() {
                        cstack.push(SquareMatrix::default());
                    }
                },

                Display() => screen.display(),
                Save(filename) => screen.write(&filename).unwrap(),

                &Translate(x, y, z) => {
                    let mut tr = SquareMatrix::new_translate(x, y, z);
                    tr.apply_rcs(cstack);
                    cstack.pop();
                    cstack.push(tr);
                },

                &Scale(x, y, z) => {
                    let mut tr = SquareMatrix::new_scale(x, y, z);
                    tr.apply_rcs(cstack);
                    cstack.pop();
                    cstack.push(tr);
                },

                &Rotate(axis, degrees) => {
                    let mut tr = match axis {
                        Axis::X => SquareMatrix::new_rot_x(degrees),
                        Axis::Y => SquareMatrix::new_rot_y(degrees),
                        Axis::Z => SquareMatrix::new_rot_z(degrees),
                    };
                    tr.apply_rcs(cstack);
                    cstack.pop();
                    cstack.push(tr);
                },

                &Cuboid(x, y, z, h, w, d) => {
                    draw::add_box(&mut temp, x, y, z, w, h, d);
                    temp.apply_rcs(cstack);
                    screen.draw_polygons(&temp, light_const);
                },

                &Sphere(x, y, z, r) => {
                    draw::add_sphere(&mut temp, x, y, z, r, STEPS_3D);
                    temp.apply_rcs(cstack);
                    screen.draw_polygons(&temp, light_const);
                }

                &Torus(x, y, z, r0, r1) => {
                    draw::add_torus(&mut temp, x, y, z, r0, r1, STEPS_3D);
                    temp.apply_rcs(cstack);
                    screen.draw_polygons(&temp, light_const);
                }

                &Line(x0, y0, z0, x1, y1, z1) => {
                    draw::add_edge(&mut temp, x0, y0, z0, x1, y1, z1);
                    temp.apply_rcs(cstack);
                    screen.draw_lines(&temp, LINE_COLOR);
                }

                Constants(_) | Frames(_) | Basename(_) | Vary(..) => {},

                //_ => unimplemented!("{:?}", command),
            }
        }
    }

}

impl Default for ToDoList {
    fn default() -> Self {
        let ops = vec![];
        let symbols: SymbolTable = HashMap::new();
        ToDoList { ops, symbols }
    }
}
