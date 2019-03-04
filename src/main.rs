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

    Ok(())
}
