use crate::vector::Vector;
use crate::{
    AMBIENT_LIGHT, AMBIENT_REFLECT, DIFFUSE_REFLECT, LIGHT_COLOR, LIGHT_POS, SPECULAR_EXP,
    SPECULAR_REFLECT, VIEW_VECTOR,
};
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
            self.blue.saturating_add(rhs.blue),
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
            as_u8(f64::from(self.red) * rhs.red),
            as_u8(f64::from(self.green) * rhs.green),
            as_u8(f64::from(self.blue) * rhs.blue),
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
        Shine { red: s, green: s, blue: s }
    }

    pub fn get_shine(normal: &Vector) -> Color {
        let light = LIGHT_POS.normalized();
        let view = VIEW_VECTOR.normalized();
        let normal = normal.normalized();

        &Shine::get_ambient()
            + &Shine::get_diffuse(&normal, &light)
            + &Shine::get_specular(&normal, &light, &view)
    }

    fn get_ambient() -> Color {
        AMBIENT_LIGHT * &AMBIENT_REFLECT
    }

    fn get_diffuse(normal: &Vector, light: &Vector) -> Color {
        LIGHT_COLOR * &DIFFUSE_REFLECT * (normal.dot_product(light))
    }

    fn get_specular(normal: &Vector, light: &Vector, view: &Vector) -> Color {
        let reflected = normal * 2. * light.dot_product(normal) - light;
        let angle = reflected.dot_product(&view).powi(SPECULAR_EXP);
        LIGHT_COLOR * &SPECULAR_REFLECT * angle
    }
}

fn as_u8(f: f64) -> u8 {
    if f < 0. {
        0
    } else if f > 255. {
        255
    } else {
        f as u8
    }
}
