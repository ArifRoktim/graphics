pub mod draw;
pub mod matrix;
pub mod parse;
pub mod screen;
pub mod vector;

use matrix::SquareMatrix;
use screen::Screen;
use screen::color::{self, Color, Shine};
use std::{env, process};
use vector::Vector;

const PICTURE_DIR: &str = "out";
const XRES: usize = 500;
const YRES: usize = 500;
const PIXELS: usize = XRES * YRES;
const STEPS_2D: usize = 100;
const STEPS_3D: usize = 100;
const SPECULAR_EXP: i32 = 4;

static AMBIENT_LIGHT: Color = Color {red: 50, green: 50, blue: 50};
static LIGHT_COLOR: Color = color::CYAN;
static LIGHT_POS: Vector = Vector {x: 0.5, y: 0.75, z: 1.};
static VIEW_VECTOR: Vector = Vector {x: 0., y: 0., z: 1.};

static AMBIENT_REFLECT: Shine = Shine {red: 0.1, green: 0.1, blue: 0.1};
static DIFFUSE_REFLECT: Shine = Shine {red: 0.5, green: 0.5, blue: 0.5};
static SPECULAR_REFLECT: Shine = Shine {red: 0.5, green: 0.5, blue: 0.5};

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        eprintln!("Not enough arguments! Provide a script file!");
        process::exit(1);
    }
    let filename = &args[1];

    let mut screen = Screen::default();
    let mut cstack = vec![SquareMatrix::default()];

    parse::parse_file(filename, &mut screen, &mut cstack);
}
