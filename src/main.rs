pub mod draw;
pub mod matrix;
pub mod parse;
pub mod screen;
pub mod vector;

use matrix::Matrix;
use screen::{color, Screen};
use std::{env, process};

const XRES: usize = 700;
const YRES: usize = 700;
const STEPS_2D: usize = 100;
const STEPS_3D: usize = 20;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        eprintln!("Not enough arguments! Provide a script file!");
        process::exit(1);
    }
    let filename = &args[1];

    let mut screen = Screen::new(color::BLACK);

    let mut identity = Matrix::new(4);
    identity.ident();
    let mut cstack = Vec::new();
    cstack.push(identity);

    parse::parse_file(filename, &mut screen, &mut cstack);
}
