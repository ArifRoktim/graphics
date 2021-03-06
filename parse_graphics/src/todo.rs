use super::{Axis, Command, ParseError};
// TODO: Re-export these and instead import from super
use crate::ast::{Expression, Number, Operation as Op};
use lib_graphics::PICTURE_DIR;
use lib_graphics::{draw, Light, Matrix, MatrixMult, Reflection, Screen, SquareMatrix};
use parse_obj::ObjParser;
use std::collections::HashMap;
use std::convert::TryInto;
use std::fs;
use std::io::ErrorKind;
use std::path::PathBuf;
use std::process::Command as SubProcess;
use std::cell::RefCell;

type SymbolTable = RefCell<HashMap<String, Symbol>>;

fn evalb(expr: &Expression, symtab: Option<&SymbolTable>) -> Number {
    use Expression::*;
    use Op::*;
    if let Num(n) = expr {
        n.to_owned()
    } else if let Action(lhs, op, rhs) = expr {
        // recursive post order traversal of the Expression tree
        let lhs = evalb(lhs, symtab);
        let rhs = evalb(rhs, symtab);
        match op {
            Add => lhs + rhs,
            Divide => lhs / rhs,
            Multiply => lhs * rhs,
            Subtract => lhs - rhs,
            IntDivide => lhs.intdiv(rhs),
        }
    } else if let Var(sym) = expr {
        let sym = symtab
            .expect("Need symbol table!")
            .borrow()
            .get(sym)
            .cloned()
            .unwrap_or_else(|| panic!("Variable `{:?}` is undefined!", sym));
        if let Symbol::Num(n) = sym {
            n
        } else {
            panic!("Variable must refer to a numerical value!")
        }
    } else {
        // Expression only has the variants, Num, Action, and Var
        unreachable!();
    }
}
pub fn eval_f64(expr: &Expression, symtab: Option<&SymbolTable>) -> f64 {
    evalb(expr, symtab).into()
}
pub fn eval_usize(expr: &Expression, symtab: Option<&SymbolTable>) -> usize {
    evalb(expr, symtab).try_into().unwrap()
}


#[derive(Clone, Debug)]
pub enum Symbol {
    Constant(Reflection),
    Knob(f64),
    Num(Number),
}

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
    pub lights: Option<Vec<Light>>,
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

    pub fn add_sym(&self, k: String, v: Symbol) {
        //self.symbols.insert(k, v);
        self.symbols.borrow_mut().insert(k, v);
    }
    pub fn get_sym(&self, key: &str) -> Option<Symbol> {
        self.symbols.borrow().get(key).cloned()
    }

    pub fn add_light(&mut self, mut light: Light) -> Result<(), ParseError> {
        // push the light after normalizing its vector
        light.pos.normalize();
        // replace the None with a Some before pushing
        self.lights.get_or_insert_with(|| Vec::with_capacity(1));
        self.lights.as_mut().unwrap().push(light);
        Ok(())
    }

    fn first_pass(&self) -> Option<(usize, String)> {
        use Command::*;

        let (mut basename, mut frames) = (None, None);
        let mut vary = false;
        for operation in &self.ops {
            match &operation.command {
                Basename(s) => basename = Some(s.to_owned()),
                Frames(n) => frames = Some(n),
                Vary(..) => vary = true,
                _ => {},
            }
        }
        // TODO: Move these checks to semantic analyzer
        // If `vary` wasn't found, we're not animating
        if !vary {
            None
        }
        // If `vary` was found but `frames` wasn't, user error
        else if frames.is_none() {
            panic!("`frames` command must also be given if `vary` command given!")
        } else {
            // Let's animate. Set a default basename if applicable
            let base = "gif";
            let basename = basename.unwrap_or_else(|| {
                println!("`basename` command not found. Using {} as a default", base);
                String::from(base)
            });
            let frames = eval_usize(&frames.unwrap(), Some(&self.symbols));
            Some((frames, basename))
        }
    }

    fn second_pass(&self, frames: usize) -> Vec<HashMap<String, f64>> {
        let mut knob_table = vec![HashMap::new(); frames];
        for operation in &self.ops {
            if let Command::Vary(knob, frame_start, frame_end, val_start, val_end) =
                &operation.command
            {
                // TODO: Learn to write macros to reduce verbosity
                let frame_start: usize = eval_usize(frame_start, Some(&self.symbols));
                let frame_end: usize = eval_usize(frame_end, Some(&self.symbols));
                let val_start: f64 = eval_f64(val_start, Some(&self.symbols));
                let val_end: f64 = eval_f64(val_end, Some(&self.symbols));

                // TODO: Move these checks to semantic analyzer
                if frame_start > frame_end || frame_end > (frames - 1) {
                    panic!(
                        "Vary: start frame must be larger than end frame!
                           Start: {}, End: {}",
                        frame_start, frame_end
                    );
                }
                let diff = (val_end - val_start) / (frame_end - frame_start) as f64;
                let mut val = val_start;
                #[allow(clippy::needless_range_loop)]
                for frame in frame_start..frame_end {
                    knob_table[frame].insert(knob.to_owned(), val);
                    val += diff;
                }
            }
        }
        knob_table
    }

    #[allow(clippy::many_single_char_names)]
    pub fn run(self, screen: &mut Screen, cstack: &mut Vec<SquareMatrix>) {
        use Command::*;

        // Temporary edge/polygon matrix
        let mut draw = Matrix::default();
        // Temporary point matrix used for sphere and torus
        let mut points = Matrix::default();

        // Add variables to symbol table
        self.add_sym("XRES".into(), Symbol::Num(Number::PosInt(screen.xres)));
        self.add_sym("YRES".into(), Symbol::Num(Number::PosInt(screen.yres)));

        // Check for animation code in script
        let animation = self.first_pass();
        // get the number of frames for first_pass, or default to 1 frame otherwise
        let frames = animation.as_ref().map_or(1, |s| s.0);
        // If animating, generate the knob table
        let knob_table = animation.as_ref().map(|s| self.second_pass(s.0));
        // extract basename, consuming `animation` in process
        let basename = animation.map(|s| s.1);

        // first delete old output frames
        if let Some(base) = &basename {
            let mut path = PathBuf::from(PICTURE_DIR);
            // gif dir
            path.push(base);
            dbg!(&path);
            let status = fs::remove_dir_all(&path);
            if let Err(e) = &status {
                if e.kind() != ErrorKind::NotFound {
                    status.unwrap();
                }
            }
        }

        // Get the list of light sources
        let lights = self.lights.as_ref().unwrap_or(&screen.lights).clone();
        let lights = lights.as_slice();

        for frame in 0..frames {
            dbg!(&frame);

            if let Some(knob_table) = &knob_table {
                for (knob, val) in knob_table[frame].iter() {
                    self.add_sym(knob.to_owned(), Symbol::Knob(*val));
                }
            }

            for operation in &self.ops {
                // clear matrix for every operation
                draw.clear();
                points.clear();
                let command = &operation.command;

                // From an Option<String>, get the symbol with that name from the hashmap,
                // and extract the reflection from the Constant
                let light_const = operation
                    .light_const
                    .as_ref()
                    // TODO: Use self.symbols[s] instead to panic when symbol isnt found
                    .and_then(|s| self.get_sym(s))
                    .map(|s| match s {
                        Symbol::Constant(r) => r,
                        _ => panic!("Expected light constant!"),
                    });

                // ditto but for the knob
                let knob = match &operation.knob {
                    Some(k) => {
                        let symbol = self.get_sym(k).unwrap();
                        match symbol {
                            Symbol::Knob(v) => Some(v),
                            _ => panic!("Expected knob!"),
                        }
                    },
                    None => None,
                };

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
                    Save(filename) => screen.write(&[filename.as_str()]).unwrap(),

                    Mesh(filename) => {
                        // get the file
                        let mut file = PathBuf::from("objects");
                        file.push(filename);
                        // read the file and parse it, adding to the polygon matrix
                        let file = fs::read_to_string(file).expect("Error reading mesh file!");
                        ObjParser::load(&mut draw, &file).expect("Error parsing mesh file!");
                        // draw the polygon matrix
                        draw.apply_rcs(cstack);
                        screen.draw_polygons(&draw, light_const.as_ref(), lights);
                    },

                    Translate(x, y, z) => {
                        let (x, y, z) = (eval_f64(x, Some(&self.symbols)), eval_f64(y, Some(&self.symbols)), eval_f64(z, Some(&self.symbols)));
                        let (x, y, z) = match knob {
                            Some(k) => (x * k, y * k, z * k),
                            None => (x, y, z),
                        };
                        let mut tr = SquareMatrix::new_translate(x, y, z);
                        tr.apply_rcs(cstack);
                        cstack.pop();
                        cstack.push(tr);
                    },

                    Scale(x, y, z) => {
                        let (x, y, z) = (eval_f64(x, Some(&self.symbols)), eval_f64(y, Some(&self.symbols)), eval_f64(z, Some(&self.symbols)));
                        let (x, y, z) = match knob {
                            Some(k) => (x * k, y * k, z * k),
                            None => (x, y, z),
                        };
                        let mut tr = SquareMatrix::new_scale(x, y, z);
                        tr.apply_rcs(cstack);
                        cstack.pop();
                        cstack.push(tr);
                    },

                    Rotate(axis, degrees) => {
                        let degrees = eval_f64(degrees, Some(&self.symbols));
                        let degrees = match knob {
                            Some(k) => degrees * k,
                            None => degrees,
                        };
                        let mut tr = match axis {
                            Axis::X => SquareMatrix::new_rot_x(degrees),
                            Axis::Y => SquareMatrix::new_rot_y(degrees),
                            Axis::Z => SquareMatrix::new_rot_z(degrees),
                        };
                        tr.apply_rcs(cstack);
                        cstack.pop();
                        cstack.push(tr);
                    },

                    Cuboid(x, y, z, h, w, d) => {
                        let (x, y, z) = (eval_f64(x, Some(&self.symbols)), eval_f64(y, Some(&self.symbols)), eval_f64(z, Some(&self.symbols)));
                        let (h, w, d) = (eval_f64(h, Some(&self.symbols)), eval_f64(w, Some(&self.symbols)), eval_f64(d, Some(&self.symbols)));
                        draw::add_box(&mut draw, x, y, z, w, h, d);
                        draw.apply_rcs(cstack);
                        screen.draw_polygons(&draw, light_const.as_ref(), lights);
                    },

                    Sphere(x, y, z, r) => {
                        let (x, y, z, r) = (eval_f64(x, Some(&self.symbols)), eval_f64(y, Some(&self.symbols)), eval_f64(z, Some(&self.symbols)), eval_f64(r, Some(&self.symbols)));
                        draw::add_sphere(&mut draw, &mut points, x, y, z, r, screen.steps_3d);
                        draw.apply_rcs(cstack);
                        screen.draw_polygons(&draw, light_const.as_ref(), lights);
                    },

                    Torus(x, y, z, r0, r1) => {
                        let (x, y, z) = (eval_f64(x, Some(&self.symbols)), eval_f64(y, Some(&self.symbols)), eval_f64(z, Some(&self.symbols)));
                        let (r0, r1) = (eval_f64(r0, Some(&self.symbols)), eval_f64(r1, Some(&self.symbols)));
                        draw::add_torus(&mut draw, &mut points, x, y, z, r0, r1, screen.steps_3d);
                        draw.apply_rcs(cstack);
                        screen.draw_polygons(&draw, light_const.as_ref(), lights);
                    },

                    Line(x0, y0, z0, x1, y1, z1) => {
                        let (x0, y0, z0) = (eval_f64(x0, Some(&self.symbols)), eval_f64(y0, Some(&self.symbols)), eval_f64(z0, Some(&self.symbols)));
                        let (x1, y1, z1) = (eval_f64(x1, Some(&self.symbols)), eval_f64(y1, Some(&self.symbols)), eval_f64(z1, Some(&self.symbols)));
                        draw::add_edge(&mut draw, x0, y0, z0, x1, y1, z1);
                        draw.apply_rcs(cstack);
                        screen.draw_lines(&draw, screen.line_color);
                    },

                    Constants(_) | Frames(_) | Basename(_) | Vary(..) | Light(..) => {},
                    //_ => unimplemented!("{:?}", command),
                }
            }

            // When animating, at the end of every frame:
            if let Some(base) = &basename {
                // Save the screen
                let file_name = format!("{:03}.png", frame); // pad filename with 3 zeros
                let path = &[base.as_str(), file_name.as_str()][..];
                screen.write(path).expect("Error writing file!");

                // Reset the screen and coordinate systems
                screen.clear();
                cstack.clear();
            }
        }

        // When animating, at the end of all frames, convert the images to a gif
        if let Some(base) = &basename {
            let pic_frames = format!("{}/{}/*", PICTURE_DIR, base);
            let gif_name = format!("{}/{}.gif", PICTURE_DIR, base);
            let convert = SubProcess::new("convert")
                .arg("-delay")
                .arg("1.7")
                .arg(&pic_frames)
                .arg(&gif_name)
                .spawn();
            match convert {
                Ok(mut proc) => {
                    println!("Making gif: {}", &gif_name);
                    proc.wait().unwrap();
                },
                Err(err) => panic!(err),
            };
        }
    }
}

impl Default for ToDoList {
    fn default() -> Self {
        let ops = vec![];
        let lights = None;
        let symbols = RefCell::new(HashMap::new());
        ToDoList { ops, symbols, lights }
    }
}
