use std::fmt;

const COLS: usize = 4;
// Each point in the matrix is a row that is 4 columns wide
// For the purposes of display and multiplication, this is switched.
// Each point is then represented by a column and the rows correspond
// to either all the x values, y values, z values, etc.
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

    // Modifies matrix to become the identity matrix
    // Assumes matrix is a square matrix less than or equal to 4x4 in size
    pub fn ident(&mut self) {
        for (row_n, row) in self.m.iter_mut().enumerate() {
            match row_n {
                0 => row[0] = 1.0,
                1 => row[1] = 1.0,
                2 => row[2] = 1.0,
                3 => row[3] = 1.0,
                _ => panic!("Array isn't square or is larger than 4x4!"),
            }
        }
    }

    // Modifies other matrix to be = self * other
    // Assumes self is a 4x4 matrix
    pub fn mult(&self, other: &mut Matrix) {
        // columns and rows are switched
        // First check that both matrices can be multiplied
        if self.rows() != other.cols() {
            panic!("Can't multiply {}x{} by {}x{}!",
                   self.cols(), self.rows(), other.cols(), other.rows());
        }
        // Graphical lens
        // for each column in other, for each row in self
        for row in other.m.iter_mut() {
            let orig_other_row = row.clone();
            for self_col in 0..self.cols() {
                let mut sum = 0.0;
                for self_row in 0..self.rows() {
                    sum += self.m[self_row][self_col] * orig_other_row[self_row];
                }
                row[self_col] = sum;
            }
        }
    }

    pub fn add_point(&mut self, x: f64, y: f64, z: f64) {
        let point = [x, y, z, 1.0];
        self.m.push(point);
    }

    pub fn add_edge(&mut self, x0: f64, y0: f64, z0:f64,
                    x1: f64, y1: f64, z1: f64) {
        self.add_point(x0, y0, z0);
        self.add_point(x1, y1, z1);
    }

}

impl fmt::Display for Matrix {
    // Print 2d array so that each point is a columns
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
            x.push_str(&format!("{: <width$.prec$} ",
                                row[0], prec = prec, width = width));
            y.push_str(&format!("{: <width$.prec$} ",
                                row[1], prec = prec, width = width));
            z.push_str(&format!("{: <width$.prec$} ",
                                row[2], prec = prec, width = width));
            one.push_str(&format!("{: <width$.prec$} ",
                                row[3], prec = prec, width = width));
        }
        write!(f, "{}\n{}\n{}\n{}", x, y, z, one)
    }
}
