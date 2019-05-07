use crate::draw::{self, Curve};
use crate::matrix::{Matrix, MatrixMult, SquareMatrix};
use crate::screen::{color, Color, Screen};
use crate::{STEPS_2D, STEPS_3D};

use std::fs;
use std::io::prelude::*;
use std::process::{Command, Stdio};

const FOREGROUND: Color = color::GREEN;

pub fn parse_file(filename: &str, screen: &mut Screen, cstack: &mut Vec<SquareMatrix>) {
    let contents = match fs::read_to_string(filename) {
        Ok(contents) => contents,
        Err(error) => panic!("Error reading file \"{}\": {}", filename, error),
    };

    // Temporary edge and polygon matrices
    let mut edges = Matrix::new(0);
    let mut polygons = Matrix::new(0);

    let mut iter = contents.lines();
    while let Some(line) = iter.next() {
        edges.clear();
        polygons.clear();

        match line {
            // ignore empty lines and lines that start with #
            "" => {},
            comment if comment.starts_with('#') => {},

            "line" => draw_line(&mut edges, cstack, screen, iter.next()),
            "circle" => circle(&mut edges, cstack, screen, iter.next()),
            "hermite" => hermite(&mut edges, cstack, screen, iter.next()),
            "bezier" => bezier(&mut edges, cstack, screen, iter.next()),
            "box" => draw_box(&mut polygons, cstack, screen, iter.next()),
            "sphere" => sphere(&mut polygons, cstack, screen, iter.next()),
            "torus" => torus(&mut polygons, cstack, screen, iter.next()),
            "scale" => scale(cstack, iter.next()),
            "move" => translate(cstack, iter.next()),
            "rotate" => rotate(cstack, iter.next()),
            "save" => save(screen, iter.next()),
            "display" => display(screen),
            "clear" => screen.clear(),
            "push" => {
                // push a copy of the last item
                let copy = cstack.last().unwrap_or_default().clone();
                cstack.push(copy);
            },
            "pop" => {
                cstack.pop();
                // Make sure that the stack is never empty
                if cstack.is_empty() {
                    cstack.push(SquareMatrix::default());
                }
            },
            "ident" | "apply" => panic!("{} is a deprecated command!", line),
            // some command that's not valid or yet implemented
            _ => unimplemented!("{}", line),
        }
    }
}

fn draw_line(edges: &mut Matrix, stack: &[SquareMatrix], screen: &mut Screen, args: Option<&str>) {
    let err_msg = "Line requires 6 f64 arguments!";
    let args = args.expect(err_msg);
    // Split by whitespace, parse the `str`s into `f64`s, then collect into
    // a vector. Use &* on vector to get a slice
    #[rustfmt::skip]
    let args = &*args
        .split_whitespace()
        .map(|n| n.parse::<f64>().expect(err_msg))
        .collect::<Vec<f64>>();
    match *args {
        [x0, y0, z0, x1, y1, z1] => {
            draw::add_edge(edges, x0, y0, z0, x1, y1, z1);
            stack.last().unwrap_or_default().mult(edges);
            screen.draw_lines(edges, FOREGROUND);
        },
        _ => panic!(err_msg),
    }
}

fn circle(edges: &mut Matrix, stack: &[SquareMatrix], screen: &mut Screen, args: Option<&str>) {
    let err_msg = "Circle requires 4 f64 arguments!";
    let args = args.expect(err_msg);
    // Split by whitespace, parse the `str`s into `f64`s, then collect into
    // a vector. Use &* on vector to get a slice
    #[rustfmt::skip]
    let args = &*args
        .split_whitespace()
        .map(|n| n.parse::<f64>().expect(err_msg))
        .collect::<Vec<f64>>();
    match *args {
        [cx, cy, cz, r] => {
            draw::add_circle(edges, cx, cy, cz, r, STEPS_2D);
            stack.last().unwrap_or_default().mult(edges);
            screen.draw_lines(edges, FOREGROUND);
        },
        _ => panic!(err_msg),
    }
}

fn hermite(edges: &mut Matrix, stack: &[SquareMatrix], screen: &mut Screen, args: Option<&str>) {
    let err_msg = "Hermite requires 8 f64 arguments!";
    let args = args.expect(err_msg);
    // Split by whitespace, parse the `str`s into `f64`s, then collect into
    // a vector. Use &* on vector to get a slice
    #[rustfmt::skip]
    let args = &*args
        .split_whitespace()
        .map(|n| n.parse::<f64>().expect(err_msg))
        .collect::<Vec<f64>>();
    match *args {
        [p0x, p0y, p1x, p1y, r0x, r0y, r1x, r1y] => {
            let curve = Curve::Hermite { p0x, p0y, p1x, p1y, r0x, r0y, r1x, r1y };
            draw::add_curve(edges, &curve, STEPS_2D);
            stack.last().unwrap_or_default().mult(edges);
            screen.draw_lines(edges, FOREGROUND);
        },
        _ => panic!(err_msg),
    }
}

fn bezier(edges: &mut Matrix, stack: &[SquareMatrix], screen: &mut Screen, args: Option<&str>) {
    let err_msg = "Hermite requires 8 f64 arguments!";
    let args = args.expect(err_msg);
    // Split by whitespace, parse the `str`s into `f64`s, then collect into
    // a vector. Use &* on vector to get a slice
    #[rustfmt::skip]
    let args = &*args
        .split_whitespace()
        .map(|n| n.parse::<f64>().expect(err_msg))
        .collect::<Vec<f64>>();
    match *args {
        [p0x, p0y, p1x, p1y, p2x, p2y, p3x, p3y] => {
            let curve = Curve::Bezier { p0x, p0y, p1x, p1y, p2x, p2y, p3x, p3y };
            draw::add_curve(edges, &curve, STEPS_2D);
            stack.last().unwrap_or_default().mult(edges);
            screen.draw_lines(edges, FOREGROUND);
        },
        _ => panic!(err_msg),
    }
}

fn draw_box(polygons: &mut Matrix, stack: &[SquareMatrix], screen: &mut Screen, args: Option<&str>) {
    let err_msg = "Box requires 6 f64 args!";
    let args = args.expect(err_msg);
    #[rustfmt::skip]
    let args = &*args
        .split_whitespace()
        .map(|n| n.parse::<f64>().expect(err_msg))
        .collect::<Vec<f64>>();
    match *args {
        [x, y, z, width, height, depth] => {
            draw::add_box(polygons, x, y, z, width, height, depth);
            stack.last().unwrap_or_default().mult(polygons);
            screen.draw_polygons(polygons, FOREGROUND);
        },
        _ => panic!(err_msg),
    }
}

fn sphere(polygons: &mut Matrix, stack: &[SquareMatrix], screen: &mut Screen, args: Option<&str>) {
    let err_msg = "Sphere requires 4 f64 args!";
    let args = args.expect(err_msg);
    #[rustfmt::skip]
    let args = &*args
        .split_whitespace()
        .map(|n| n.parse::<f64>().expect(err_msg))
        .collect::<Vec<f64>>();
    match *args {
        [cx, cy, cz, r] => {
            draw::add_sphere(polygons, cx, cy, cz, r, STEPS_3D);
            stack.last().unwrap_or_default().mult(polygons);
            screen.draw_polygons(polygons, FOREGROUND);
        },
        _ => panic!(err_msg),
    }
}

fn torus(polygons: &mut Matrix, stack: &[SquareMatrix], screen: &mut Screen, args: Option<&str>) {
    let err_msg = "Torus requires 5 f64 args!";
    let args = args.expect(err_msg);
    #[rustfmt::skip]
    let args = &*args
        .split_whitespace()
        .map(|n| n.parse::<f64>().expect(err_msg))
        .collect::<Vec<f64>>();
    match *args {
        [cx, cy, cz, minor_r, major_r] => {
            draw::add_torus(polygons, cx, cy, cz, minor_r, major_r, STEPS_3D);
            stack.last().unwrap_or_default().mult(polygons);
            screen.draw_polygons(polygons, FOREGROUND);
        },
        _ => panic!(err_msg),
    }
}

fn scale(stack: &mut Vec<SquareMatrix>, args: Option<&str>) {
    let err_msg = "Scale requires 3 f64 arguments!";
    let args = args.expect(err_msg);
    // Split by whitespace, parse the `str`s into `f64`s, then collect into
    // a vector. Use &* on vector to get a slice
    #[rustfmt::skip]
    let args = &*args
        .split_whitespace()
        .map(|n| n.parse::<f64>().expect(err_msg))
        .collect::<Vec<f64>>();
    match *args {
        [sx, sy, sz] => {
            let mut tr = SquareMatrix::new_scale(sx, sy, sz);
            stack.last().unwrap_or_default().mult(&mut tr);
            stack.pop();
            stack.push(tr);
        },
        _ => panic!(err_msg),
    }
}

fn translate(stack: &mut Vec<SquareMatrix>, args: Option<&str>) {
    let err_msg = "Move requires 3 f64 arguments!";
    let args = args.expect(err_msg);
    // Split by whitespace, parse the `str`s into `f64`s, then collect into
    // a vector. Use &* on vector to get a slice
    #[rustfmt::skip]
    let args = &*args
        .split_whitespace()
        .map(|n| n.parse::<f64>().expect(err_msg))
        .collect::<Vec<f64>>();
    match *args {
        [tx, ty, tz] => {
            let mut tr = SquareMatrix::new_translate(tx, ty, tz);
            stack.last().unwrap_or_default().mult(&mut tr);
            stack.pop();
            stack.push(tr);
        },
        _ => panic!(err_msg),
    }
}

fn rotate(stack: &mut Vec<SquareMatrix>, args: Option<&str>) {
    let err_msg = "Rotate requires an axis ('x', 'y', or 'z') and a f64!";
    let args = args.expect(err_msg);
    let args: Vec<&str> = args.split_whitespace().collect();
    let theta: f64 = args.get(1).expect(err_msg).parse().expect(err_msg);
    let mut tr = match args[0] {
        "x" => SquareMatrix::new_rot_x(theta),
        "y" => SquareMatrix::new_rot_y(theta),
        "z" => SquareMatrix::new_rot_z(theta),
        _ => panic!(err_msg),
    };
    stack.last().unwrap_or_default().mult(&mut tr);
    stack.pop();
    stack.push(tr);
}

fn save(screen: &Screen, args: Option<&str>) {
    let err_msg = "Save requires a file name!";
    let args = args.expect(err_msg);
    screen.write(args).unwrap();
}

fn display(screen: &Screen) {
    if let Ok(mut proc) = Command::new("display").stdin(Stdio::piped()).spawn() {
        #[rustfmt::skip]
        proc.stdin
            .as_mut()
            .unwrap()
            .write_all(screen.to_string().as_bytes())
            .unwrap();
        proc.wait().unwrap();
    } else {
        eprintln!("Error running `display` command! Is Image Magick installed?");
    }
}
