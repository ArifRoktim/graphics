pub mod draw;
pub mod matrix;
pub mod parse;
pub mod screen;
pub mod vector;

pub use matrix::{Matrix, SquareMatrix, IDENTITY};
pub use screen::{consts, Color, Screen, Shine};
pub use vector::Vector;

const PICTURE_DIR: &str = "out";

// TODO: XRES, YRES, PIXELS should be properties of the screen.
const XRES: usize = 500;
const YRES: usize = 500;
const PIXELS: usize = XRES * YRES;

// TODO: These could be passed as arguments to parse::parse_file()
const STEPS_2D: usize = 100;
const STEPS_3D: usize = 500;

// TODO: These will be replaced with user inputed lighting values via the script
const SPECULAR_EXP: i32 = 8;
static AMBIENT_LIGHT: Color = Color::new(50, 50, 50);
static LIGHT_COLOR: Color = consts::CYAN;
static LIGHT_POS: Vector = Vector::new(0.5, 0.75, 1.);
static VIEW_VECTOR: Vector = Vector::new(0., 0., 1.);
static AMBIENT_REFLECT: Shine = Shine::new(0.1, 0.1, 0.1);
static DIFFUSE_REFLECT: Shine = Shine::new(0.5, 0.5, 0.5);
static SPECULAR_REFLECT: Shine = Shine::new(0.5, 0.5, 0.5);
