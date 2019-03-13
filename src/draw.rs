use std::f64::consts::PI;
use crate::matrix::Matrix;

const TOTAL_STEPS: i32 = 100;
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

