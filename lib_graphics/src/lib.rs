pub mod draw;
pub mod matrix;
pub mod screen;
pub mod vector;

pub use matrix::{Matrix, MatrixMult, SquareMatrix, IDENTITY};
pub use screen::{color, Color, Reflection, Screen, Shine};
pub use vector::Vector;

pub const PICTURE_DIR: &str = "out";

#[derive(Debug, Clone)]
pub struct Light {
    pub pos: Vector,
    pub color: Color,
}
impl Light {
    pub const fn new(pos: Vector, color: Color) -> Light {
        Light { pos, color }
    }
}
