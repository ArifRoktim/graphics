pub mod draw;
pub mod matrix;
pub mod parse;
pub mod screen;

use matrix::Matrix;
use screen::{Color, Screen};
use std::env;
use std::process;

const XRES: usize = 700;
const YRES: usize = 700;
const STEPS_2D: i32 = 100;
const STEPS_3D: i32 = 20;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        eprintln!("Not enough arguments! Provide a script file!");
        process::exit(1);
    }
    let filename = &args[1];

    let mut screen = Screen::new(Color::black());

    let mut edges = Matrix::new(0);
    let mut polygons = Matrix::new(0);
    let mut transform = Matrix::new(4);

    parse::parse_file(
        filename,
        &mut screen,
        &mut edges,
        &mut polygons,
        &mut transform,
    );
}
