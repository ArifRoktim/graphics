pub mod matrix;
pub mod screen;
pub mod parse;
pub mod draw;

use matrix::Matrix;
use screen::{Screen,Color};
use std::env;
use std::process;

const COLUMNS: usize = 500;
const ROWS: usize = 500;

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
