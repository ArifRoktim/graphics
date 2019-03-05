use crate::screen::{Color,Screen};
use crate::matrix::Matrix;

use std::io::prelude::*;
use std::fs;
use std::process::{Command,Stdio};

pub fn parse_file(filename: &str, screen: &mut Screen,
                  edges: &mut Matrix, transform: &mut Matrix) {
    let contents = match fs::read_to_string(filename) {
        Ok(contents) => contents,
        Err(error) => panic!("Error reading file \"{}\": {}", filename, error),
    };

    let mut iter = contents.lines();
    while let Some(line) = iter.next() {
        match line {
            "line" => draw_line(edges, iter.next()),
            "scale" => scale(transform, iter.next()),
            "move" => translate(transform, iter.next()),
            "rotate" => rotate(transform, iter.next()),
            "save" => save(screen, iter.next()),
            "display" => {
                draw_lines(screen, edges, Color::purple());
                display(screen);
            }
            "ident" => transform.ident(),
            "apply" => transform.mult(edges),
            "" => {},
            _ => panic!("\"{}\" not yet implemented!", line),
        }
    }
}

fn save(screen: &Screen, args: Option<&str>) {
    let err_msg = "Save requires a file name!";
    let args = args.expect(err_msg);
    screen.write(args).unwrap();
}

fn draw_line(edges: &mut Matrix, args: Option<&str>) {
    let err_msg = "Line requires 6 f64 arguments!";
    let args = args.expect(err_msg);
    // Split by whitespace, parse the `str`s into `f64`s, then collect into
    // a vector. Use &* on vector to get a slice
    let args = &* args.split_whitespace()
        .map(|n| n.parse::<f64>().expect(err_msg))
        .collect::<Vec<f64>>();
    match args {
        [x0, y0, z0, x1, y1, z1] => {
            edges.add_edge(*x0, *y0, *z0, *x1, *y1, *z1);
        },
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

fn scale(transform: &mut Matrix, args: Option<&str>) {
    let err_msg = "Scale requires 3 f64 arguments!";
    let args = args.expect(err_msg);
    // Split by whitespace, parse the `str`s into `f64`s, then collect into
    // a vector. Use &* on vector to get a slice
    let args = &* args.split_whitespace()
        .map(|n| n.parse::<f64>().expect(err_msg))
        .collect::<Vec<f64>>();
    match args {
        [sx, sy, sz] => {
            Matrix::new_scale(*sx, *sy, *sz).mult(transform);
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
    match args {
        [tx, ty, tz] => {
            Matrix::new_translate(*tx, *ty, *tz).mult(transform);
        }
        _ => panic!(err_msg),
    }
}

fn draw_lines(screen: &mut Screen, edges: &mut Matrix, c: Color) {
    // clear screen
    screen.fill(Color::black());
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

// line: add a line to the point matrix.
// takes 6 arguemnts (x0, y0, z0, x1, y1, z1)
//
// display: clear the screen, draw the lines of the point matrix to 
// the screen, display the screen.
//
// ident: set the transform matrix to the identity matrix.
//
// scale: create a scale matrix, then multiply the transform matrix by it.
// takes 3 arguments (sx, sy, sz)
//
// apply: apply the current transformation matrix to the edge matrix
//
// move: create a translation matrix, then multiply the transform matrix
// by the translation matrix.
// takes 3 arguments (tx, ty, tz)
//
// rotate: create a rotation matrix, then multiply the transform matrix
// by the rotation matrix.
// takes 2 arguments (axis theta)
//
// save: clear the screen, draw the lines of the point matrix to the
// screen/frame save the screen/frame to a file.
// takes 1 argument (file name)
