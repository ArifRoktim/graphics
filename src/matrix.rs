use std::fmt;

const COLS: usize = 4;
// Each point in the matrix is a row that is 4 columns wide
// For the purposes of display and multiplication, this is switched.
// Each point is then represented by a column and the rows correspond
// to either all the x values, y values, z values, etc.
#[derive(Debug, Clone)]
pub struct Matrix {
    pub m: Vec<[f64; 4]>,
}

impl Matrix {
    pub fn rows(&self) -> usize {
        self.m.len()
    }

    pub fn cols(&self) -> usize {
        COLS
    }
}

impl Matrix {
    pub fn new(rows: usize) -> Matrix {
        let mut m: Vec<[f64; COLS]> = Vec::new();
        for _ in 0..rows {
            m.push([0.0; 4]);
        }
        Matrix { m }
    }

    pub fn new_translate(x: f64, y: f64, z: f64) -> Matrix {
        // Translation matrix:
        // [1, 0, 0, x]
        // [0, 1, 0, y]
        // [0, 0, 1, z]
        // [0, 0, 0, 1]
        let m = &[
            [1., 0., 0., 0.],
            [0., 1., 0., 0.],
            [0., 0., 1., 0.],
            [x, y, z, 1.],
        ][..];
        Matrix::from(m)
    }

    pub fn new_scale(x: f64, y: f64, z: f64) -> Matrix {
        // scale matrix:
        // [a, 0, 0, 0]
        // [0, b, 0, 0]
        // [0, 0, c, 0]
        // [0, 0, 0, 1]
        let m = &[
            [x, 0., 0., 0.],
            [0., y, 0., 0.],
            [0., 0., z, 0.],
            [0., 0., 0., 1.],
        ][..];
        Matrix::from(m)
    }

    pub fn new_rot_x(theta: f64) -> Matrix {
        // theta(θ) is in degrees
        // rotation_x matrix:
        // [1, 0, 0, 0]
        // [0, cosθ, -sinθ, 0]
        // [0, sinθ, cosθ, 0]
        // [0, 0, 0, 1]
        let radians = theta.to_radians();
        let (sin, cos) = radians.sin_cos();
        let m = &[
            [1., 0., 0., 0.],
            [0., cos, sin, 0.],
            [0., -1. * sin, cos, 0.],
            [0., 0., 0., 1.],
        ][..];
        Matrix::from(m)
    }

    pub fn new_rot_y(theta: f64) -> Matrix {
        // theta(θ) is in degrees
        // rotation_y matrix:
        // [cosθ, 0, sinθ, 0]
        // [0, 1, 0, 0]
        // [-sinθ, 0, cosθ, 0]
        // [0, 0, 0, 1]
        let radians = theta.to_radians();
        let (sin, cos) = radians.sin_cos();
        let m = &[
            [cos, 0., -1. * sin, 0.],
            [0., 1., 0., 0.],
            [sin, 0., cos, 0.],
            [0., 0., 0., 1.],
        ][..];
        Matrix::from(m)
    }

    pub fn new_rot_z(theta: f64) -> Matrix {
        // theta(θ) is in degrees
        // rotation_z matrix:
        // [cosθ, -sinθ, 0, 0]
        // [sinθ, cosθ, 0, 0]
        // [0, 0, 1, 0]
        // [0, 0, 0, 1]
        let radians = theta.to_radians();
        let (sin, cos) = radians.sin_cos();
        let m = &[
            [cos, sin, 0., 0.],
            [-1. * sin, cos, 0., 0.],
            [0., 0., 1., 0.],
            [0., 0., 0., 1.],
        ][..];
        Matrix::from(m)
    }

    pub fn clear(&mut self) {
        self.m.clear();
    }

    pub fn push(&mut self, point: [f64; 4]) {
        self.m.push(point);
    }

    // Modifies a square matrix to become the identity matrix
    pub fn ident(&mut self) {
        if self.rows() != self.cols() {
            panic!("Can't call method ident() on a non-square matrix!");
        }
        for (row_n, row) in self.m.iter_mut().enumerate() {
            for (col_n, col) in row.iter_mut().enumerate() {
                if row_n == col_n {
                    *col = 1.;
                } else {
                    *col = 0.;
                }
            }
        }
    }

    // Modifies other matrix to be = self * other
    pub fn mult(&self, other: &mut Matrix) {
        // columns and rows are switched
        // First check that both matrices can be multiplied
        if self.rows() != other.cols() {
            panic!(
                "Can't multiply {}x{} by {}x{}!",
                self.cols(),
                self.rows(),
                other.cols(),
                other.rows()
            );
        }
        // Graphical lens
        // for each column in other, for each row in self
        for row in other.m.iter_mut() {
            let orig_other_row = *row;
            for self_col in 0..self.cols() {
                let mut sum = 0.0;
                for self_row in 0..self.rows() {
                    sum += self.m[self_row][self_col] * orig_other_row[self_row];
                }
                row[self_col] = sum;
            }
        }
    }
}

impl From<&[[f64; 4]]> for Matrix {
    fn from(matrix: &[[f64; 4]]) -> Matrix {
        let mut ret = Matrix::new(0);
        for &row in matrix {
            ret.m.push(row);
        }
        ret
    }
}

impl fmt::Display for Matrix {
    // Print 2d array so that each point is a column
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        // precision of the floating point
        let prec: usize = 2;
        // extra padding to make numbers line up
        let width: usize = 8;
        // Allocate strings large enough to hold each row.
        // Each row has an f64 that has a min width of 9 chars (padding + space)
        // but go above that just to make sure there aren't an reallocations
        let size: usize = (width + prec + 2) * (self.cols() + 2);
        let mut x = String::with_capacity(size);
        let mut y = String::with_capacity(size);
        let mut z = String::with_capacity(size);
        let mut one = String::with_capacity(size);

        for row in &self.m {
            x.push_str(&format!(
                "{: <width$.prec$} ",
                row[0],
                prec = prec,
                width = width
            ));
            y.push_str(&format!(
                "{: <width$.prec$} ",
                row[1],
                prec = prec,
                width = width
            ));
            z.push_str(&format!(
                "{: <width$.prec$} ",
                row[2],
                prec = prec,
                width = width
            ));
            one.push_str(&format!(
                "{: <width$.prec$} ",
                row[3],
                prec = prec,
                width = width
            ));
        }
        write!(f, "{}\n{}\n{}\n{}", x, y, z, one)
    }
}
