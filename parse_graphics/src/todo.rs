use super::{Axis, Command, ParseError};
// TODO: Re-export these and instead import from super
use crate::ast::{Expression, Number};
use lib_graphics::PICTURE_DIR;
use lib_graphics::{draw, Light, Matrix, MatrixMult, Reflection, Screen, SquareMatrix};
use parse_obj::ObjParser;
use std::collections::HashMap;
use std::convert::TryInto;
use std::fs;
use std::path::PathBuf;
use std::process::Command as SubProcess;

fn evalb(expr: &Expression) -> Number {
    use Expression::*;
    match expr {
        Num(n) => n.to_owned(),
        Action(..) => unimplemented!("Expression action not yet done!"),
    }
}
fn eval_f64(expr: &Expression) -> f64 {
    evalb(expr).into()
}
fn eval_usize(expr: &Expression) -> usize {
    evalb(expr).try_into().unwrap()
}


#[derive(Debug)]
pub enum Symbol {
    Constant(Reflection),
    Knob(f64),
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
    pub symbols: HashMap<String, Symbol>,
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

    pub fn add_sym(&mut self, k: String, v: Symbol) {
        self.symbols.insert(k, v);
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
        use Expression::*;

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
            let frames = eval_usize(&frames.unwrap());
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
                let frame_start: usize = eval_usize(frame_start);
                let frame_end: usize = eval_usize(frame_end);
                let val_start: f64 = eval_f64(val_start);
                let val_end: f64 = eval_f64(val_end);

                // TODO: Move these checks to semantic analyzer
                if frame_start > frame_end || frame_end > frames {
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
    pub fn run(mut self, screen: &mut Screen, cstack: &mut Vec<SquareMatrix>) {
        use Command::*;

        // Temporary edge/polygon matrix
        let mut draw = Matrix::default();
        // Temporary point matrix used for sphere and torus
        let mut points = Matrix::default();

        // Check for animation code in script
        let animation = self.first_pass();
        // get the number of frames for first_pass, or default to 1 frame otherwise
        let frames = animation.as_ref().map_or(1, |s| s.0);
        // If animating, generate the knob table
        let knob_table = animation.as_ref().map(|s| self.second_pass(s.0));
        // extract basename, consuming `animation` in process
        let basename = animation.map(|s| s.1);

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
                    .and_then(|s| self.symbols.get(s))
                    .map(|s| match s {
                        Symbol::Constant(r) => r,
                        _ => panic!("Expected light constant!"),
                    });

                // ditto but for the knob
                let knob = match &operation.knob {
                    Some(k) => {
                        let symbol = &self.symbols[k];
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
                        screen.draw_polygons(&draw, light_const, lights);
                    },

                    Translate(x, y, z) => {
                        let (x, y, z) = (eval_f64(x), eval_f64(y), eval_f64(z));
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
                        let (x, y, z) = (eval_f64(x), eval_f64(y), eval_f64(z));
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
                        let degrees = eval_f64(degrees);
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
                        let (x, y, z) = (eval_f64(x), eval_f64(y), eval_f64(z));
                        let (h, w, d) = (eval_f64(h), eval_f64(w), eval_f64(d));
                        draw::add_box(&mut draw, x, y, z, w, h, d);
                        draw.apply_rcs(cstack);
                        screen.draw_polygons(&draw, light_const, lights);
                    },

                    Sphere(x, y, z, r) => {
                        let (x, y, z, r) = (eval_f64(x), eval_f64(y), eval_f64(z), eval_f64(r));
                        draw::add_sphere(&mut draw, &mut points, x, y, z, r, screen.steps_3d);
                        draw.apply_rcs(cstack);
                        screen.draw_polygons(&draw, light_const, lights);
                    },

                    Torus(x, y, z, r0, r1) => {
                        let (x, y, z) = (eval_f64(x), eval_f64(y), eval_f64(z));
                        let (r0, r1) = (eval_f64(r0), eval_f64(r1));
                        draw::add_torus(&mut draw, &mut points, x, y, z, r0, r1, screen.steps_3d);
                        draw.apply_rcs(cstack);
                        screen.draw_polygons(&draw, light_const, lights);
                    },

                    Line(x0, y0, z0, x1, y1, z1) => {
                        let (x0, y0, z0) = (eval_f64(x0), eval_f64(y0), eval_f64(z0));
                        let (x1, y1, z1) = (eval_f64(x1), eval_f64(y1), eval_f64(z1));
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
        let symbols = HashMap::new();
        ToDoList { ops, symbols, lights }
    }
}
