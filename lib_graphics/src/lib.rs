pub mod draw;
pub mod matrix;
pub mod screen;
pub mod vector;

pub use matrix::{Matrix, MatrixMult, SquareMatrix, IDENTITY};
pub use screen::{color, Color, Reflection, Screen, Shine};
pub use vector::Vector;

use color::consts;
// static/consts exports
pub const PICTURE_DIR: &str = "out";
// TODO: These should be properties of the screen.
pub const XRES: usize = 500;
pub const YRES: usize = 500;
pub const PIXELS: usize = XRES * YRES;
pub const LINE_COLOR: Color = consts::BLACK;
pub const STEPS_2D: usize = 100;
pub const STEPS_3D: usize = 200;

// TODO: These will be replaced with user inputed lighting values via the script
pub const SPECULAR_EXP: i32 = 6;
pub static AMBIENT_LIGHT: Color = Color::new(50, 50, 50);
pub const LIGHT: Light = Light::new(Vector::new(0.5, 0.75, 1.), consts::WHITE);
pub static VIEW_VECTOR: Vector = Vector::new(0., 0., 1.);
pub static REFLECT: Reflection = Reflection::new(
    Shine::new(0.1, 0.1, 0.1), // Ambient
    Shine::new(0.5, 0.5, 0.5), // Diffuse
    Shine::new(0.5, 0.5, 0.5), // Specular
);

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
