pub mod draw;
pub mod matrix;
pub mod screen;
pub mod vector;
pub use matrix::{Matrix, SquareMatrix, IDENTITY};
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
pub const STEPS_3D: usize = 100;

// TODO: These will be replaced with user inputed lighting values via the script
pub const SPECULAR_EXP: i32 = 8;
pub static AMBIENT_LIGHT: Color = Color::new(50, 50, 50);
pub static LIGHT_COLOR: Color = consts::WHITE;
pub static LIGHT_POS: Vector = Vector::new(0.5, 0.75, 1.);
pub static VIEW_VECTOR: Vector = Vector::new(0., 0., 1.);
pub static AMBIENT_REFLECT: Shine = Shine::new(0.1, 0.1, 0.1);
pub static DIFFUSE_REFLECT: Shine = Shine::new(0.5, 0.5, 0.5);
pub static SPECULAR_REFLECT: Shine = Shine::new(0.5, 0.5, 0.5);
