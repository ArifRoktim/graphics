use std::error::Error;

pub mod matrix;
// needed by screen module
use matrix::Matrix;
pub mod screen;

//use screen::Screen;
//use screen::Color;

const COLUMNS: usize = 500;
const ROWS: usize = 500;

fn main() -> Result<(), Box<dyn Error>> {
    let mut edges = Matrix::new(0);

    edges.add_edge(0.0, 0.0, 0.0, 100.0, 100.0, 100.0);
    println!("Printing edge list:\n{}", edges);

    let t0 = Matrix::new_translate(10.0, 20.0, 30.0);
    t0.mult(&mut edges);
    println!("Applying translation(10, 20, 30)\
             \nPrinting edge list:\n{}", edges);

    let t0 = Matrix::new_scale(2., 3., 4.);
    t0.mult(&mut edges);
    println!("Applying scale(2, 3, 4)\
             \nPrinting edge list:\n{}", edges);

    Ok(())
}
