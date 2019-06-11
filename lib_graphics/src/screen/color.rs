use crate::{Light, Screen, Vector};
use std::f64;
use std::fmt;
use std::ops::{Add, AddAssign, Mul};

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

// TODO: Remove `Copy` trait
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

// TODO: Replace multiple `impl` blocks by using `Borrow<T>` trait
impl AddAssign for Color {
    fn add_assign(&mut self, rhs: Color) {
        self.red = self.red.saturating_add(rhs.red);
        self.green = self.green.saturating_add(rhs.green);
        self.blue = self.blue.saturating_add(rhs.blue);
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

    pub fn get_shine(
        screen: &Screen,
        normal: &Vector,
        reflect: Option<&Reflection>,
        lights: &[Light],
    ) -> Color {
        assert!(!lights.is_empty(), "Must have at least 1 light!");

        // If `reflect` isn't provided use the default
        let reflect = reflect.unwrap_or(&screen.reflection);

        let normal_v = Vector::normalized(normal);
        Shine::get_ambient(screen.ambient_light, &reflect.ambient)
            + &Shine::get_diffuse(&normal_v, lights, &reflect.diffuse)
            + &Shine::get_specular(&normal_v, lights, &screen.view_vector, &reflect.specular, screen.specular_exp)
    }

    fn get_ambient(ambient_light: Color, reflect: &Shine) -> Color {
        ambient_light * reflect
    }

    fn get_diffuse(normal_v: &Vector, lights: &[Light], reflect: &Shine) -> Color {
        let mut diffuse = Color::default();
        for light in lights {
            diffuse += light.color * light.pos.dot_product(normal_v);
        }
        diffuse * reflect
    }

    fn get_specular(
        normal_v: &Vector,
        lights: &[Light],
        view_v: &Vector,
        reflect: &Shine,
        specular_exp: i32,
    ) -> Color {
        //let reflected = normal_v * 2. * light.pos.dot_product(normal_v) - &light.pos;
        //let angle = match reflected.dot_product(&view_v) {
        //    neg if neg < 0. => 0.,
        //    others => others.powi(SPECULAR_EXP),
        //};
        //light.color * reflect * angle;
        let mut specular = Color::default();
        for light in lights {
            let reflected = normal_v * 2. * light.pos.dot_product(normal_v) - &light.pos;
            let angle = match reflected.dot_product(&view_v) {
                neg if neg < 0. => 0.,
                others => others.powi(specular_exp),
            };
            specular += light.color * angle;
        }
        specular * reflect
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

impl Reflection {
    pub const fn new(ambient: Shine, diffuse: Shine, specular: Shine) -> Reflection {
        Reflection { ambient, diffuse, specular }
    }
}
