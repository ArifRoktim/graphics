use crate::{AMBIENT_LIGHT, AMBIENT_REFLECT, DIFFUSE_REFLECT, LIGHT_COLOR, LIGHT_POS, SPECULAR_EXP, SPECULAR_REFLECT, VIEW_VECTOR};
use crate::vector::Vector;
use rand::Rng;
use std::f64;
use std::fmt;
use std::ops::{Add, Mul};

pub const BLACK: Color = Color { red: 0, green: 0, blue: 0 };
pub const WHITE: Color = Color { red: 255, green: 255, blue: 255 };
pub const RED: Color = Color { red: 255, green: 0, blue: 0 };
pub const GREEN: Color = Color { red: 0, green: 255, blue: 0 };
pub const BLUE: Color = Color { red: 0, green: 0, blue: 255 };
pub const YELLOW: Color = Color { red: 255, green: 255, blue: 0 };
pub const PURPLE: Color = Color { red: 255, green: 0, blue: 255 };
pub const CYAN: Color = Color { red: 0, green: 255, blue: 255 };

#[derive(Debug, Default, Copy, Clone)]
pub struct Color {
    pub red: u8,
    pub green: u8,
    pub blue: u8,
}

impl Color {
    pub fn new(r: u8, g: u8, b: u8) -> Color {
        Color { red: r, green: g, blue: b }
    }

    pub fn rand() -> Color {
        let mut rng = rand::thread_rng();
        Color { red: rng.gen(), green: rng.gen(), blue: rng.gen() }
    }

    pub fn color(&mut self, c: Color) {
        self.red = c.red;
        self.green = c.green;
        self.blue = c.blue;
    }
}

impl fmt::Display for Color {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{} {} {} ", self.red, self.green, self.blue)
    }
}

impl Add<&Self> for Color {
    type Output = Color;
    fn add(self, rhs: &Color) -> Color {
        &self + rhs
    }
}

impl Add for &Color {
    type Output = Color;
    fn add(self, rhs: &Color) -> Color {
        Color::new(
            self.red.saturating_add(rhs.red),
            self.green.saturating_add(rhs.green),
            self.blue.saturating_add(rhs.blue)
        )
    }
}

impl Mul<&Shine> for Color {
    type Output = Color;
    fn mul(self, rhs: &Shine) -> Color {
        &self * rhs
    }
}

impl Mul<&Shine> for &Color {
    type Output = Color;
    fn mul(self, rhs: &Shine) -> Color {
        Color::new(
            (f64::from(self.red) * rhs.red) as u8,
            (f64::from(self.green) * rhs.green) as u8,
            (f64::from(self.blue) * rhs.blue) as u8
        )
    }
}

impl Mul<f64> for Color {
    type Output = Color;
    fn mul(self, rhs: f64) -> Color {
        &self * rhs
    }
}

impl Mul<f64> for &Color {
    type Output = Color;
    fn mul(self, rhs: f64) -> Color {
        self * &Shine::new(rhs)
    }
}

#[derive(Debug, Default)]
pub struct Shine {
    pub red: f64,
    pub green: f64,
    pub blue: f64,
}

impl Shine {
    pub fn new(s: f64) -> Shine {
        Shine {red: s, green: s, blue: s}
    }

    // color get_lighting( double *normal, double *view, color alight, double light[2][3], double *areflect, double *dreflect, double *sreflect) {
    pub fn get_shine(normal: &Vector) -> Color {
        &Shine::get_ambient() + &Shine::get_diffuse(normal) + &Shine::get_specular(normal)
    }

    fn get_ambient() -> Color {
        AMBIENT_LIGHT * &AMBIENT_REFLECT
    }

    fn get_diffuse(normal: &Vector) -> Color {
        LIGHT_COLOR * &DIFFUSE_REFLECT * (normal.norm().dot_product(&LIGHT_POS.norm()))
    }

    fn get_specular(normal: &Vector) -> Color {
        Color::default()
    }
}
//color calculate_ambient(color alight, double *areflect ) {
//color calculate_diffuse(double light[2][3], double *dreflect, double *normal ) {
//color calculate_specular(double light[2][3], double *sreflect, double *view, double *normal ) {
