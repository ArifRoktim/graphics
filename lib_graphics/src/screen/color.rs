use crate::vector::Vector;
use crate::{
    AMBIENT_LIGHT, AMBIENT_REFLECT, DIFFUSE_REFLECT, LIGHT_COLOR, LIGHT_POS, SPECULAR_EXP,
    SPECULAR_REFLECT, VIEW_VECTOR,
};
use std::f64;
use std::fmt;
use std::ops::{Add, Mul};

pub mod consts {
    use super::Color;

    pub const BLACK: Color = Color::new(0, 0, 0);
    pub const WHITE: Color = Color::new(255, 255, 255);
    pub const RED: Color = Color::new(255, 0, 0);
    pub const GREEN: Color = Color::new(0, 255, 0);
    pub const BLUE: Color = Color::new(0, 0, 255);
    pub const YELLOW: Color = Color::new(255, 255, 0);
    pub const PURPLE: Color = Color::new(255, 0, 255);
    pub const CYAN: Color = Color::new(0, 255, 255);
}

#[derive(Debug, Default, Copy, Clone)]
pub struct Color {
    pub red: u8,
    pub green: u8,
    pub blue: u8,
}

impl Color {
    pub const fn new(r: u8, g: u8, b: u8) -> Color {
        Color { red: r, green: g, blue: b }
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

impl Add<&Self> for Color {
    type Output = Color;
    #[allow(clippy::op_ref)]
    fn add(self, rhs: &Color) -> Color {
        &self + rhs
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

impl Mul<&Shine> for Color {
    type Output = Color;
    #[allow(clippy::op_ref)]
    fn mul(self, rhs: &Shine) -> Color {
        &self * rhs
    }
}

impl Mul<f64> for &Color {
    type Output = Color;
    fn mul(self, rhs: f64) -> Color {
        Color::new(
            as_u8(f64::from(self.red) * rhs),
            as_u8(f64::from(self.green) * rhs),
            as_u8(f64::from(self.blue) * rhs),
        )
    }
}

impl Mul<f64> for Color {
    type Output = Color;
    #[allow(clippy::op_ref)]
    fn mul(self, rhs: f64) -> Color {
        &self * rhs
    }
}

#[derive(Debug, Default)]
pub struct Shine {
    pub red: f64,
    pub green: f64,
    pub blue: f64,
}

impl Shine {
    pub const fn new(r: f64, g: f64, b: f64) -> Shine {
        Shine { red: r, green: g, blue: b }
    }

    pub fn from_triple(s: &[f64; 9]) -> [Shine; 3] {
        let ambient = Shine::new(s[0], s[3], s[6]);
        let diffuse = Shine::new(s[1], s[4], s[7]);
        let reflective = Shine::new(s[2], s[5], s[8]);
        [ambient, diffuse, reflective]
    }

    pub fn get_shine(normal: &Vector, shine: Option<&Reflection>) -> Color {
        let light = Vector::normalized(&LIGHT_POS);
        let view = Vector::normalized(&VIEW_VECTOR);
        let normal = Vector::normalized(normal);


        let ambient = shine.map(|s| &s.ambient).unwrap_or(&AMBIENT_REFLECT);
        let diffuse = shine.map(|s| &s.diffuse).unwrap_or(&DIFFUSE_REFLECT);
        let specular = shine.map(|s| &s.specular).unwrap_or(&SPECULAR_REFLECT);
        &Shine::get_ambient(ambient)
            + &Shine::get_diffuse(&normal, &light, diffuse)
            + &Shine::get_specular(&normal, &light, &view, specular)
    }

    fn get_ambient(reflect: &Shine) -> Color {
        AMBIENT_LIGHT * reflect
    }

    fn get_diffuse(normal: &Vector, light: &Vector, reflect: &Shine) -> Color {
        LIGHT_COLOR * reflect * (normal.dot_product(light))
    }

    fn get_specular(normal: &Vector, light: &Vector, view: &Vector, reflect: &Shine) -> Color {
        let reflected = normal * 2. * light.dot_product(normal) - light;
        let angle = match reflected.dot_product(&view) {
            neg if neg < 0. => 0.,
            others => others.powi(SPECULAR_EXP),
        };
        LIGHT_COLOR * reflect * angle
    }
}

fn as_u8(f: f64) -> u8 {
    if f < 0. {
        0
    } else {
        f as u8
    }
}

#[derive(Debug)]
pub struct Reflection {
    pub ambient: Shine,
    pub diffuse: Shine,
    pub specular: Shine,
}
