use crate::matrix::COLS;
use std::ops::{Mul, Sub};

#[derive(Debug, Clone)]
pub struct Vector {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

impl Vector {
    pub const fn new(x: f64, y: f64, z: f64) -> Vector {
        Vector { x, y, z }
    }

    pub fn normalized(v: &Vector) -> Vector {
        let mut new = v.clone();
        new.normalize();
        new
    }

    pub fn normalize(&mut self) {
        let magnitude = (self.x.powi(2) + self.y.powi(2) + self.z.powi(2)).sqrt();
        self.x /= magnitude;
        self.y /= magnitude;
        self.z /= magnitude;
    }

    pub fn dot_product(&self, other: &Vector) -> f64 {
        self.x * other.x + self.y * other.y + self.z * other.z
    }

    #[rustfmt::skip]
    pub fn cross_product(&self, other: &Vector) -> Vector {
        Vector::new(
            self.y * other.z - self.z * other.y,
            self.z * other.x - self.x * other.z,
            self.x * other.y - self.y * other.x,
        )
    }

    pub fn calculate_normal(triangle: &[[f64; COLS]]) -> Vector {
        assert_eq!(3, triangle.len(), "Triangles must have 3 points!");

        let a = Vector::from(&triangle[1][..3]) - &Vector::from(&triangle[0][..3]);
        let b = Vector::from(&triangle[2][..3]) - &Vector::from(&triangle[0][..3]);

        a.cross_product(&b)
    }
}

impl From<&[f64]> for Vector {
    fn from(vector: &[f64]) -> Vector {
        assert_eq!(3, vector.len(), "Triangles must have 3 points!");

        Vector::new(vector[0], vector[1], vector[2])
    }
}

impl Sub<&Self> for Vector {
    type Output = Vector;
    fn sub(self, rhs: &Vector) -> Vector {
        &self - rhs
    }
}

impl Sub for &Vector {
    type Output = Vector;
    fn sub(self, rhs: &Vector) -> Vector {
        Vector::new(self.x - rhs.x, self.y - rhs.y, self.z - rhs.z)
    }
}

impl Mul<f64> for Vector {
    type Output = Vector;
    fn mul(self, rhs: f64) -> Vector {
        &self * rhs
    }
}

impl Mul<f64> for &Vector {
    type Output = Vector;
    fn mul(self, rhs: f64) -> Vector {
        Vector::new(self.x * rhs, self.y * rhs, self.z * rhs)
    }
}
