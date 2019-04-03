use std::f64::consts::PI;
use crate::matrix::Matrix;

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
    edges.push(point);
}

pub fn add_edge(edges: &mut Matrix, x0: f64, y0: f64, z0:f64,
                x1: f64, y1: f64, z1: f64) {
    add_point(edges, x0, y0, z0);
    add_point(edges, x1, y1, z1);
}

pub fn add_polygon(polygons: &mut Matrix,
                   x0: f64, y0: f64, z0:f64,
                   x1: f64, y1: f64, z1: f64,
                   x2: f64, y2: f64, z2: f64) {
    add_point(polygons, x0, y0, z0);
    add_point(polygons, x1, y1, z1);
    add_point(polygons, x2, y2, z2);
}

pub fn add_circle(edges: &mut Matrix, cx: f64, cy: f64, cz: f64,
                  r: f64, steps: i32){
    // Draw a circle using parametric equations
    // x_prev, y_prec, cz, and the 1st point given to `add_edge`
    let mut x_prev = cx + r;
    let mut y_prev = cy;
    // t goes from 0 -> TOTAL_STEPS in increments of `step`
    let mut t = 0;
    while t <= steps {
        let theta = f64::from(t) / f64::from(steps);
        let (sin, cos) = (2.0 * PI * theta).sin_cos();
        let x_next = r * cos + cx;
        let y_next = r * sin + cy;

        add_edge(edges, x_prev, y_prev, cz, x_next, y_next, cz);

        x_prev = x_next;
        y_prev = y_next;

        t += 1;
    }
}

pub fn add_curve(edges: &mut Matrix, curve: &Curve, steps: i32) {
    let (mut x_prev, mut y_prev) = match *curve {
        Curve::Hermite {p0x, p0y, ..} => (p0x, p0y),
        Curve::Bezier  {p0x, p0y, ..} => (p0x, p0y),
    };
    let (xs, ys) = curve.gen_coefs();

    let mut t = 0;
    while t <= steps {
        let progress = f64::from(t) / f64::from(steps);
        let x_next = xs.m[0][0] * progress.powi(3)
            + xs.m[0][1] * progress.powi(2)
            + xs.m[0][2] * progress
            + xs.m[0][3];
        let y_next = ys.m[0][0] * progress.powi(3)
            + ys.m[0][1] * progress.powi(2)
            + ys.m[0][2] * progress
            + ys.m[0][3];

        add_edge(edges, x_prev, y_prev, 0.0, x_next, y_next, 0.0);

        x_prev = x_next;
        y_prev = y_next;

        t += 1;
    }
}

pub fn add_box(polygons: &mut Matrix, x: f64, y: f64, z: f64,
               width: f64, height: f64, depth: f64) {
    let x1 = x + width;
    let y1 = y - height;
    let z1 = z - depth;

    // front face
    add_polygon(polygons, x, y, z, x1, y1, z, x, y1, z);
    add_polygon(polygons, x, y, z, x1, y, z, x1, y1, z);
    // back face
    add_polygon(polygons, x1, y, z1, x, y1, z1, x1, y1, z1);
    add_polygon(polygons, x1, y, z1, x, y, z1, x, y1, z1);
    // left face
    add_polygon(polygons, x, y1, z, x, y1, z1, x, y, z1);
    add_polygon(polygons, x, y1, z, x, y, z1, x, y, z);
    // right face
    add_polygon(polygons, x1, y1, z1, x1, y1, z, x1, y, z);
    add_polygon(polygons, x1, y1, z1, x1, y, z, x1, y, z1);
    // top face
    add_polygon(polygons, x1, y, z, x, y, z, x, y, z1);
    add_polygon(polygons, x1, y, z, x, y, z1, x1, y, z1);
    // bottom face
    add_polygon(polygons, x1, y1, z1, x, y1, z1, x, y1, z);
    add_polygon(polygons, x1, y1, z1, x, y1, z, x1, y1, z);
}

pub fn gen_sphere(cx: f64, cy: f64, cz: f64, r: f64, steps: i32) -> Matrix {
    // Matrix of the points of the surface of a sphere
    let mut points = Matrix::new(0);

    // For 0->2PI draw a semi circle that's rotated phi degrees along x axis
    let mut t_phi = 0;
    while t_phi <= steps {
        let phi = f64::from(t_phi) / f64::from(steps);
        let (sin_phi, cos_phi) = (2.0 * PI * phi).sin_cos();

        // Draw a semicircle
        let mut t_theta = 0;
        while t_theta <= steps {
            let theta = f64::from(t_theta) / f64::from(steps);
            let (sin_theta, cos_theta) = (PI * theta).sin_cos();

            let point = [r * cos_theta + cx,
                r * sin_theta * cos_phi + cy,
                r * sin_theta * sin_phi + cz,
                1.0];
            points.push(point);

            t_theta += 1;
        }

        t_phi += 1;
    }

    points
}

pub fn add_sphere(polygons: &mut Matrix, cx: f64, cy: f64, cz: f64,
                  r: f64, steps: i32) {
    let points = gen_sphere(cx, cy, cz, r, steps);

    let end: usize = steps as usize;
    let steps: usize = steps as usize + 1;

    for lat in 0..end {
        for longt in 0..end {

            let p0 = lat * steps + longt;
            let p1 = p0 + 1;
            let p2 = (p1 + steps) % (steps * (steps - 1));
            let p3 = (p0 + steps) % (steps * (steps - 1));

            if longt != steps - 2 {
                add_polygon(
                    polygons,
                    points.m[p0][0], points.m[p0][1], points.m[p0][2],
                    points.m[p1][0], points.m[p1][1], points.m[p1][2],
                    points.m[p2][0], points.m[p2][1], points.m[p2][2],
                    );
            }
            if longt != 0 {
                add_polygon(
                    polygons,
                    points.m[p0][0], points.m[p0][1], points.m[p0][2],
                    points.m[p2][0], points.m[p2][1], points.m[p2][2],
                    points.m[p3][0], points.m[p3][1], points.m[p3][2],
                    );
            }
        }
    }
}

pub fn gen_torus(cx: f64, cy: f64, cz: f64, minor_r: f64, major_r: f64, steps: i32) -> Matrix {
    // Matrix of the points of the surface of a sphere
    let mut points = Matrix::new(0);

    // For phi: 0->2PI, draw a circle of radius `minor_r` that is translated by 
    // `major_r` in the x axis and rotated phi degrees in the y axis
    let mut t_phi = 0;
    while t_phi <= steps {
        let phi = f64::from(t_phi) / f64::from(steps);
        let (sin_phi, cos_phi) = (2.0 * PI * phi).sin_cos();

        // Draw a circle
        let mut t_theta = 0;
        while t_theta <= steps {
            let theta = f64::from(t_theta) / f64::from(steps);
            let (sin_theta, cos_theta) = (2.0 * PI * theta).sin_cos();

            let point = [
                cos_phi * (minor_r * cos_theta + major_r) + cx,
                minor_r * sin_theta + cy,
                -1.0 * sin_phi * (minor_r * cos_theta + major_r) + cz,
                1.0
            ];
            points.push(point);

            t_theta += 1;
        }

        t_phi += 1;
    }

    points
}

pub fn add_torus(polygons: &mut Matrix, cx: f64, cy: f64, cz: f64,
                  minor_r: f64, major_r: f64, steps: i32) {
    let points = gen_torus(cx, cy, cz, minor_r, major_r, steps);

    let end: usize = steps as usize;
    let steps: usize = steps as usize + 1;

    for lat in 0..end {
        for longt in 0..=end {
            let p0 = lat * steps + longt;
            let p1 = p0 + 1;
            let p2 = (p1 + steps) % (steps * (steps - 1));
            let p3 = (p0 + steps) % (steps * (steps - 1));

            add_polygon(
                polygons,
                points.m[p0][0], points.m[p0][1], points.m[p0][2],
                points.m[p1][0], points.m[p1][1], points.m[p1][2],
                points.m[p2][0], points.m[p2][1], points.m[p2][2],
                );
            add_polygon(
                polygons,
                points.m[p0][0], points.m[p0][1], points.m[p0][2],
                points.m[p2][0], points.m[p2][1], points.m[p2][2],
                points.m[p3][0], points.m[p3][1], points.m[p3][2],
                );
        }
    }
}
