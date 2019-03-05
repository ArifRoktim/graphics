use crate::screen::Screen;
use crate::matrix::Matrix;

use std::fs;
use std::process;

enum Axis {
    X,
    Y,
    Z,
}

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
            //"display" => 
            _ => panic!("\"{}\" not yet implemented!", line),
        }
    }

}

fn draw_line(edges: &mut Matrix, args: Option<&str>) {
    let args = args.expect("No arguments provided for `line` command!");
    // Split by whitespace, parse the `str`s into `f64`s, then collect into
    // a vector. Use &* on vector to get a slice
    let args = &* args.split_whitespace()
        .map(|n| n.parse::<f64>().unwrap())
        .collect::<Vec<f64>>();
    match args {
        [x0, y0, z0, x1, y1, z1] => {
            edges.add_edge(*x0, *y0, *z0, *x1, *y1, *z1);
        },
        _ => panic!("Line requires 6 arguments!"),
    }
}

// line: add a line to the point matrix.
// takes 6 arguemnts (x0, y0, z0, x1, y1, z1)
//
// scale: create a scale matrix, then multiply the transform matrix by it.
// takes 3 arguments (sx, sy, sz)
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
//
// ident: set the transform matrix to the identity matrix.
//
// apply: apply the current transformation matrix to the edge matrix
//
// display: clear the screen, draw the lines of the point matrix to 
// the screen, display the screen.
