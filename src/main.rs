pub mod draw;
pub mod matrix;
pub mod parse;
pub mod screen;
pub mod vector;

use matrix::IDENTITY;
use screen::color::{self, Color, Shine};
use screen::Screen;
use std::{env, process};
use vector::Vector;

const PICTURE_DIR: &str = "out";
const XRES: usize = 500;
const YRES: usize = 500;
const PIXELS: usize = XRES * YRES;
const STEPS_2D: usize = 100;
const STEPS_3D: usize = 100;
const SPECULAR_EXP: i32 = 4;

static AMBIENT_LIGHT: Color = Color::new(50, 50, 50);
static LIGHT_COLOR: Color = color::CYAN;
static LIGHT_POS: Vector = Vector::new(0.5, 0.75, 1.);
static VIEW_VECTOR: Vector = Vector::new(0., 0., 1.);

static AMBIENT_REFLECT: Shine = Shine::new(0.1, 0.1, 0.1);
static DIFFUSE_REFLECT: Shine = Shine::new(0.5, 0.5, 0.5);
static SPECULAR_REFLECT: Shine = Shine::new(0.5, 0.5, 0.5);

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        eprintln!("Not enough arguments! Provide a script file!");
        process::exit(1);
    }
    let filename = &args[1];

    let mut screen = Screen::default();
    let mut cstack = vec![IDENTITY];

    parse::parse_file(filename, &mut screen, &mut cstack);
}
