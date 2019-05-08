use std::fmt;

pub const COLS: usize = 4;
#[rustfmt::skip]
pub const IDENTITY: SquareMatrix = SquareMatrix {
    m: [
        [1., 0., 0., 0.],
        [0., 1., 0., 0.],
        [0., 0., 1., 0.],
        [0., 0., 0., 1.],
    ]
};

// Each point in the matrix is a row that is 4 columns wide
// For the purposes of display and multiplication, this is switched.
// Each point is then represented by a column and the rows correspond
// to either all the x values, y values, z values, etc.
pub trait MatrixMult {
    // TODO: Make a RawMatrix trait for just these four methods
    // TODO: Add a transpose method for RawMatrix and use it in the from methods
    fn raw(&self) -> &[[f64; COLS]];
    fn raw_mut(&mut self) -> &mut [[f64; COLS]];
    fn cols(&self) -> usize {
        COLS
    }
    fn rows(&self) -> usize {
        self.raw().len()
    }

    // TODO: Make a seperate MatrixMult that is blanket implemented on RawMatrix
    // Modifies other matrix to be = self * other
    #[allow(clippy::needless_range_loop)]
    fn mult(&self, other: &mut MatrixMult) {
        // columns and rows are switched
        // First check that both matrices can be multiplied
        // Graphical lens: LEFT.cols == RIGHT.rows
        // Implementation: self.rows == other.cols
        assert_eq!(
            self.rows(),
            other.cols(),
            "Can't multiply {}x{} by {}x{}!",
            self.cols(),
            self.rows(),
            other.cols(),
            other.rows()
        );
        // Graphical lens: for each column in other, for each row      in self
        // Implementation: for each row    in other, for each column   in self
        for row in other.raw_mut().iter_mut() {
            let orig_other_row = *row;
            for self_col in 0..self.cols() {
                let mut sum = 0.0;
                for self_row in 0..self.rows() {
                    sum += self.raw()[self_row][self_col] * orig_other_row[self_row];
                }
                row[self_col] = sum;
            }
        }
    }
}

#[derive(Debug, Clone)]
pub struct Matrix {
    pub m: Vec<[f64; COLS]>,
}

impl Matrix {
    pub fn new(rows: usize) -> Matrix {
        // TODO: Use Vec::with_capacity here
        // TODO: Also write a seperate Matrix::with_capacity method
        let mut m: Vec<[f64; COLS]> = Vec::new();
        for _ in 0..rows {
            m.push([0.0; COLS]);
        }
        Matrix { m }
    }

    pub fn clear(&mut self) {
        self.m.clear();
    }

    pub fn push(&mut self, point: [f64; COLS]) {
        self.m.push(point);
    }
}

impl MatrixMult for Matrix {
    fn raw(&self) -> &[[f64; COLS]] {
        self.m.as_slice()
    }

    fn raw_mut(&mut self) -> &mut [[f64; COLS]] {
        self.m.as_mut_slice()
    }
}

impl From<&[[f64; COLS]]> for Matrix {
    fn from(matrix: &[[f64; COLS]]) -> Matrix {
        // TODO: Replace 0 with matrix.len()
        let mut ret = Matrix::new(0);
        for &row in matrix {
            ret.m.push(row);
        }
        ret
    }
}

// TODO: Blanked implement fmt::Display for RawMatrix
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
            x.push_str(&format!("{: <width$.prec$} ", row[0], prec = prec, width = width));
            y.push_str(&format!("{: <width$.prec$} ", row[1], prec = prec, width = width));
            z.push_str(&format!("{: <width$.prec$} ", row[2], prec = prec, width = width));
            one.push_str(&format!("{: <width$.prec$} ", row[3], prec = prec, width = width));
        }
        write!(f, "{}\n{}\n{}\n{}", x, y, z, one)
    }
}

// TODO: Implement Default for Matrix to be an empty matrix

#[derive(Debug, Clone)]
pub struct SquareMatrix {
    m: [[f64; COLS]; COLS],
}

impl SquareMatrix {
    pub fn new_translate(x: f64, y: f64, z: f64) -> SquareMatrix {
        // Translation matrix:
        // [1, 0, 0, x]
        // [0, 1, 0, y]
        // [0, 0, 1, z]
        // [0, 0, 0, 1]
        #[rustfmt::skip]
        let m = [
            [1., 0., 0., 0.],
            [0., 1., 0., 0.],
            [0., 0., 1., 0.],
            [x, y, z, 1.],
        ];
        SquareMatrix::from(m)
    }

    pub fn new_scale(x: f64, y: f64, z: f64) -> SquareMatrix {
        // scale matrix:
        // [a, 0, 0, 0]
        // [0, b, 0, 0]
        // [0, 0, c, 0]
        // [0, 0, 0, 1]
        #[rustfmt::skip]
        let m = [
            [x, 0., 0., 0.],
            [0., y, 0., 0.],
            [0., 0., z, 0.],
            [0., 0., 0., 1.],
        ];
        SquareMatrix::from(m)
    }

    pub fn new_rot_x(theta: f64) -> SquareMatrix {
        // theta(θ) is in degrees
        // rotation_x matrix:
        // [1, 0, 0, 0]
        // [0, cosθ, -sinθ, 0]
        // [0, sinθ, cosθ, 0]
        // [0, 0, 0, 1]
        let radians = theta.to_radians();
        let (sin, cos) = radians.sin_cos();
        #[rustfmt::skip]
        let m = [
            [1., 0., 0., 0.],
            [0., cos, sin, 0.],
            [0., -1. * sin, cos, 0.],
            [0., 0., 0., 1.],
        ];
        SquareMatrix::from(m)
    }

    pub fn new_rot_y(theta: f64) -> SquareMatrix {
        // theta(θ) is in degrees
        // rotation_y matrix:
        // [cosθ, 0, sinθ, 0]
        // [0, 1, 0, 0]
        // [-sinθ, 0, cosθ, 0]
        // [0, 0, 0, 1]
        let radians = theta.to_radians();
        let (sin, cos) = radians.sin_cos();
        #[rustfmt::skip]
        let m = [
            [cos, 0., -1. * sin, 0.],
            [0., 1., 0., 0.],
            [sin, 0., cos, 0.],
            [0., 0., 0., 1.],
        ];
        SquareMatrix::from(m)
    }

    pub fn new_rot_z(theta: f64) -> SquareMatrix {
        // theta(θ) is in degrees
        // rotation_z matrix:
        // [cosθ, -sinθ, 0, 0]
        // [sinθ, cosθ, 0, 0]
        // [0, 0, 1, 0]
        // [0, 0, 0, 1]
        let radians = theta.to_radians();
        let (sin, cos) = radians.sin_cos();
        #[rustfmt::skip]
        let m = [
            [cos, sin, 0., 0.],
            [-1. * sin, cos, 0., 0.],
            [0., 0., 1., 0.],
            [0., 0., 0., 1.],
        ];
        SquareMatrix::from(m)
    }
}

impl MatrixMult for SquareMatrix {
    fn raw(&self) -> &[[f64; COLS]] {
        &self.m
    }

    fn raw_mut(&mut self) -> &mut [[f64; COLS]] {
        &mut self.m
    }
}

impl From<[[f64; COLS]; COLS]> for SquareMatrix {
    fn from(matrix: [[f64; COLS]; COLS]) -> SquareMatrix {
        SquareMatrix { m: matrix }
    }
}

impl Default for SquareMatrix {
    // return the identity matrix
    fn default() -> SquareMatrix {
        self::IDENTITY
    }
}

impl Default for &SquareMatrix {
    // return the identity matrix
    fn default() -> &'static SquareMatrix {
        &self::IDENTITY
    }
}
