pub mod matrix;
pub mod screen;
pub mod parse;
pub mod draw;

use matrix::Matrix;
use screen::{Screen,Color};
use std::env;
use std::process;

const XRES: usize = 500;
const YRES: usize = 500;
const TOTAL_STEPS: i32 = 100;
const STEP: i32 = 2;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        eprintln!("Not enough arguments! Provide a script file!");
        process::exit(1);
    }
    let filename = &args[1];

    let mut screen = Screen::new(Color::black());

    let mut edges = Matrix::new(0);
    let mut transform = Matrix::new(4);

    parse::parse_file(filename, &mut screen, &mut edges, &mut transform);
}
