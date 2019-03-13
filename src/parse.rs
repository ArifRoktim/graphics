use crate::screen::{Color,Screen};
use crate::matrix::Matrix;
use crate::draw::{self, Curve};

use std::io::prelude::*;
use std::fs;
use std::process::{Command,Stdio};

const STEP: f64 = 1.0;

pub fn parse_file(filename: &str, screen: &mut Screen,
                  edges: &mut Matrix, transform: &mut Matrix) {
    let contents = match fs::read_to_string(filename) {
        Ok(contents) => contents,
        Err(error) => panic!("Error reading file \"{}\": {}", filename, error),
    };

    let mut iter = contents.lines();
    while let Some(line) = iter.next() {
        match line {
            // ignore empty lines and lines that start with #
            "" => {},
            comment if comment.starts_with('#') => {},

            "line"    => draw_line(edges, iter.next()),
            "circle"  =>    circle(edges, iter.next()),
            "hermite" =>   hermite(edges, iter.next()),
            "bezier"  =>    bezier(edges, iter.next()),
            "scale"   =>     scale(transform, iter.next()),
            "move"    => translate(transform, iter.next()),
            "rotate"  =>    rotate(transform, iter.next()),
            "save"    => save(screen, iter.next()),
            "display" => {
                draw_lines(screen, edges, Color::green());
                display(screen);
            }
            "ident" => transform.ident(),
            "apply" => transform.mult(edges),
            // some command that's not valid or yet implemented
            _ => panic!("\"{}\" not yet implemented!", line),
        }
    }
}

fn draw_line(edges: &mut Matrix, args: Option<&str>) {
    let err_msg = "Line requires 6 f64 arguments!";
    let args = args.expect(err_msg);
    // Split by whitespace, parse the `str`s into `f64`s, then collect into
    // a vector. Use &* on vector to get a slice
    let args = &* args.split_whitespace()
        .map(|n| n.parse::<f64>().expect(err_msg))
        .collect::<Vec<f64>>();
    match *args {
        [x0, y0, z0, x1, y1, z1] => {
            draw::add_edge(edges, x0, y0, z0, x1, y1, z1);
        },
        _ => panic!(err_msg),
    }
}

fn circle(edges: &mut Matrix, args: Option<&str>) {
    let err_msg = "Circle requires 4 f64 arguments!";
    let args = args.expect(err_msg);
    // Split by whitespace, parse the `str`s into `f64`s, then collect into
    // a vector. Use &* on vector to get a slice
    let args = &* args.split_whitespace()
        .map(|n| n.parse::<f64>().expect(err_msg))
        .collect::<Vec<f64>>();
    match *args {
        [cx, cy, cz, r] => {
            draw::add_circle(edges, cx, cy, cz, r, STEP);
        },
        _ => panic!(err_msg),
    }
}

fn hermite(edges: &mut Matrix, args: Option<&str>) {
    let err_msg = "Hermite requires 8 f64 arguments!";
    let args = args.expect(err_msg);
    // Split by whitespace, parse the `str`s into `f64`s, then collect into
    // a vector. Use &* on vector to get a slice
    let args = &* args.split_whitespace()
        .map(|n| n.parse::<f64>().expect(err_msg))
        .collect::<Vec<f64>>();
    match *args {
        [p0x, p0y, p1x, p1y, r0x, r0y, r1x, r1y] => {
            let curve = Curve::Hermite {p0x, p0y, p1x, p1y, r0x, r0y, r1x, r1y};
            draw::add_curve(edges, &curve, STEP);
        },
        _ => panic!(err_msg),
    }
}

fn bezier(edges: &mut Matrix, args: Option<&str>) {
    let err_msg = "Hermite requires 8 f64 arguments!";
    let args = args.expect(err_msg);
    // Split by whitespace, parse the `str`s into `f64`s, then collect into
    // a vector. Use &* on vector to get a slice
    let args = &* args.split_whitespace()
        .map(|n| n.parse::<f64>().expect(err_msg))
        .collect::<Vec<f64>>();
    match *args {
        [p0x, p0y, p1x, p1y, p2x, p2y, p3x, p3y] => {
            let curve = Curve::Bezier {p0x, p0y, p1x, p1y, p2x, p2y, p3x, p3y};
            draw::add_curve(edges, &curve, STEP);
        },
        _ => panic!(err_msg),
    }
}

fn scale(transform: &mut Matrix, args: Option<&str>) {
    let err_msg = "Scale requires 3 f64 arguments!";
    let args = args.expect(err_msg);
    // Split by whitespace, parse the `str`s into `f64`s, then collect into
    // a vector. Use &* on vector to get a slice
    let args = &* args.split_whitespace()
        .map(|n| n.parse::<f64>().expect(err_msg))
        .collect::<Vec<f64>>();
    match *args {
        [sx, sy, sz] => {
            Matrix::new_scale(sx, sy, sz).mult(transform);
        }
        _ => panic!(err_msg),
    }
}

fn translate(transform: &mut Matrix, args: Option<&str>) {
    let err_msg = "Move requires 3 f64 arguments!";
    let args = args.expect(err_msg);
    // Split by whitespace, parse the `str`s into `f64`s, then collect into
    // a vector. Use &* on vector to get a slice
    let args = &* args.split_whitespace()
        .map(|n| n.parse::<f64>().expect(err_msg))
        .collect::<Vec<f64>>();
    match *args {
        [tx, ty, tz] => {
            Matrix::new_translate(tx, ty, tz).mult(transform);
        }
        _ => panic!(err_msg),
    }
}

fn rotate(transform: &mut Matrix, args: Option<&str>) {
    let err_msg = "Rotate requires an axis ('x', 'y', or 'z') and a f64!";
    let args = args.expect(err_msg);
    let args: Vec<&str> = args.split_whitespace().collect();
    let theta: f64 = args.get(1).expect(err_msg)
        .parse().expect(err_msg);
    match args[0] {
        "x" => Matrix::new_rot_x(theta).mult(transform),
        "y" => Matrix::new_rot_y(theta).mult(transform),
        "z" => Matrix::new_rot_z(theta).mult(transform),
        _ => panic!(err_msg),
    }
}

fn save(screen: &Screen, args: Option<&str>) {
    let err_msg = "Save requires a file name!";
    let args = args.expect(err_msg);
    screen.write(args).unwrap();
}

fn draw_lines(screen: &mut Screen, edges: &mut Matrix, c: Color) {
    // clear screen
    screen.clear();
    // ignore possible error
    let _ = screen.draw_lines(edges, c);
}

fn display(screen: &Screen) {
    if let Ok(mut proc) = Command::new("display").stdin(Stdio::piped()).spawn() {
        proc.stdin.as_mut().unwrap()
            .write_all(screen.to_string().as_bytes())
            .unwrap();
        proc.wait().unwrap();
    } else {
        eprintln!("Error running `display` command! Is Image Magick installed?");
    }
}
