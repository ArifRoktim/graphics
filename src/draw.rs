use std::f64::consts::PI;
use crate::matrix::Matrix;

const TOTAL_STEPS: i32 = 100;

pub enum Curve {
    Hermite {p0x: f64, p0y: f64, p1x: f64, p1y: f64,
             r0x: f64, r0y: f64, r1x: f64, r1y: f64},
    Bezier  {p0x: f64, p0y: f64, p1x: f64, p1y: f64,
             p2x: f64, p2y: f64, p3x: f64, p3y: f64},
}

impl Curve {
    pub fn gen_coefs(&self) -> (Matrix, Matrix) {
        let mult = self.gen_coef_helper();
        let mut coefs_x;
        let mut coefs_y;
        match *self {
            Curve::Hermite {p0x, p0y, p1x, p1y, r0x, r0y, r1x, r1y} => {
                // [p0]
                // [p1]
                // [r0]
                // [r1]
                coefs_x = Matrix::from(&[
                    [p0x, p1x, r0x, r1x],
                ][..]);
                coefs_y = Matrix::from(&[
                    [p0y, p1y, r0y, r1y],
                ][..]);
            },
            Curve::Bezier {p0x, p0y, p1x, p1y, p2x, p2y, p3x, p3y} => {
                // [p0]
                // [p1]
                // [p2]
                // [p3]
                coefs_x = Matrix::from(&[
                    [p0x, p1x, p2x, p3x],
                ][..]);
                coefs_y = Matrix::from(&[
                    [p0y, p1y, p2y, p3y],
                ][..]);
            }
        }
        mult.mult(&mut coefs_x);
        mult.mult(&mut coefs_y);
        (coefs_x, coefs_y)
    }

    fn gen_coef_helper(&self) -> Matrix {
        match self {
            Curve::Hermite {..} => {
                // [2, -2, 1, 1]
                // [-3, 3, -2, -1]
                // [0, 0, 1, 0]
                // [1, 0, 0, 0]
                let m = &[
                    [2., -3., 0., 1.,],
                    [-2., 3., 0., 0.,],
                    [1., -2., 1., 0.,],
                    [1., -1., 0., 0.,],
                ][..];
                Matrix::from(m)
            },
            Curve::Bezier {..} => {
                // [-1, 3, -3, 1]
                // [3, -6, 3, 0]
                // [-3, 3, 0, 0]
                // [1, 0, 0, 0]
                let m = &[
                    [-1., 3., -3., 1.,],
                    [3., -6., 3., 0.,],
                    [-3., 3., 0., 0.,],
                    [1., 0., 0., 0.,],
                ][..];
                Matrix::from(m)
            },
        }
    }

}

fn add_point(edges: &mut Matrix, x: f64, y: f64, z: f64) {
    let point = [x, y, z, 1.0];
    edges.m.push(point);
}

pub fn add_edge(edges: &mut Matrix, x0: f64, y0: f64, z0:f64,
                x1: f64, y1: f64, z1: f64) {
    add_point(edges, x0, y0, z0);
    add_point(edges, x1, y1, z1);
}

pub fn add_circle(edges: &mut Matrix, cx: f64, cy: f64, cz: f64, r: f64, step: f64){
    // Draw a circle using parametric equations
    // x_prev, y_prec, cz, and the 1st point given to `add_edge`
    let mut x_prev = cx + r;
    let mut y_prev = cy;
    // t goes from 0 -> TOTAL_STEPS in increments of `step`
    let mut t = 0.0;
    while (t as i32) <= TOTAL_STEPS {
        let theta = t / TOTAL_STEPS as f64;
        let (sin, cos) = (2.0 * PI * theta).sin_cos();
        let x_next = r * cos + cx;
        let y_next = r * sin + cy;

        add_edge(edges, x_prev, y_prev, cz, x_next, y_next, cz);

        x_prev = x_next;
        y_prev = y_next;

        t += step;
    }
}

pub fn add_curve(edges: &mut Matrix, curve: &Curve, step: f64) {
    let (mut x_prev, mut y_prev) = match *curve {
        Curve::Hermite {p0x, p0y, ..} => (p0x, p0y),
        Curve::Bezier  {p0x, p0y, ..} => (p0x, p0y),
    };
    let (xs, ys) = curve.gen_coefs();

    let mut t = 0.0f64;
    while (t as i32) <= TOTAL_STEPS {
        let x_next = xs.m[0][0] * (t / TOTAL_STEPS as f64).powi(3)
            + xs.m[0][1] * (t / TOTAL_STEPS as f64).powi(2)
            + xs.m[0][2] * (t / TOTAL_STEPS as f64)
            + xs.m[0][3];
        let y_next = ys.m[0][0] * (t / TOTAL_STEPS as f64).powi(3)
            + ys.m[0][1] * (t / TOTAL_STEPS as f64).powi(2)
            + ys.m[0][2] * (t / TOTAL_STEPS as f64)
            + ys.m[0][3];

        add_edge(edges, x_prev, y_prev, 0.0, x_next, y_next, 0.0);

        x_prev = x_next;
        y_prev = y_next;

        t += step;
    }
}
