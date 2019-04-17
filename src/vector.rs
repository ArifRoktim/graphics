use crate::matrix::COLS;

pub struct Vector {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

impl Vector {
    pub fn new(x: f64, y: f64, z: f64) -> Vector {
        Vector { x, y, z }
    }

    pub fn normalize(&mut self) {
        let magnitude = (self.x.powi(2) + self.y.powi(2) + self.z.powi(2)).sqrt();
        self.x /= magnitude;
        self.y /= magnitude;
        self.z /= magnitude;
    }

    pub fn dot_product(&self, other: &Vector) -> f64 {
        self.x * other.x + self.y * other.y + self.z + other.z
    }

    #[rustfmt::skip]
    pub fn calculate_normal(triangle: &[[f64; COLS]]) -> Vector {
        let a = Vector::new(
            triangle[1][0] - triangle[0][0],
            triangle[1][1] - triangle[0][1],
            triangle[1][2] - triangle[0][2],
        );
        let b = Vector::new(
            triangle[2][0] - triangle[0][0],
            triangle[2][1] - triangle[0][1],
            triangle[2][2] - triangle[0][2],
        );

        Vector::new(
            a.y * b.z - a.z * b.y,
            a.z * b.x - a.x * b.z,
            a.x * b.y - a.y * b.x,
        )
    }
}
