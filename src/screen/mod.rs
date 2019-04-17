use crate::matrix::Matrix;
use crate::vector::Vector;
use crate::{PIXELS, XRES, YRES};
use std::f64;
use std::fmt;
use std::fs::File;
use std::io::{self, prelude::*};
use std::path::Path;
use std::process::Command;

pub mod color;
pub use color::Color;

pub struct Screen {
    pub pixels: Box<[[Color; XRES]; YRES]>,
    pub zbuffer: Box<[[f64; XRES]; YRES]>,
    pub color: Color,
}

impl Screen {
    pub fn new(c: Color) -> Screen {
        Screen {
            pixels: Box::new([[color::BLACK; XRES]; YRES]),
            zbuffer: Box::new([[f64::NEG_INFINITY; XRES]; YRES]),
            color: c,
        }
    }

    pub fn blank() -> Screen {
        Self::new(color::BLACK)
    }

    pub fn write_ppm(&self, f: &str) -> io::Result<()> {
        let mut file = File::create(f)?;
        file.write_all(self.to_string().as_bytes())
    }

    pub fn write(&self, f: &str) -> io::Result<()> {
        let base = Path::new(f).file_stem();
        match base {
            Some(base) => {
                let ppm = base.to_str().unwrap().to_owned() + ".ppm";
                self.write_ppm(&ppm)?;
                if let Ok(mut proc) = Command::new("convert").arg(ppm).arg(f).spawn() {
                    proc.wait().unwrap();
                } else {
                    eprintln!(
                        "Error running `convert` command! \
                         Is Image Magick installed?"
                    );
                }
            },
            None => panic!("Please specify a file name!"),
        }
        Ok(())
    }

    pub fn clear(&mut self) {
        self.fill(self.color);
    }

    pub fn fill(&mut self, c: Color) {
        for row in self.pixels.iter_mut() {
            for pixel in row.iter_mut() {
                pixel.color(c)
            }
        }
    }

    pub fn draw_point(&mut self, px: i32, py: i32, c: Color) {
        if px < 0 || px >= (XRES as i32) || py < 0 || py >= (YRES as i32) {
            return;
        }
        // Cast the coordinates to `usize` and also
        // make (0, 0) the bottom left corner instead of the top left corner
        let (px, py) = (px as usize, YRES - 1 - (py as usize));
        // Get the pixel at point p and set its color
        self.pixels[py][px].color(c);
    }

    // Bresenham's line algorithm
    #[allow(clippy::many_single_char_names)]
    pub fn draw_line(&mut self, x0: i32, y0: i32, x1: i32, y1: i32, c: Color) {
        // swap points if going right -> left
        if x0 > x1 {
            self.draw_line(x1, y1, x0, y0, c);
            return;
        }

        let (mut x, mut y) = (x0, y0);
        let a = 2 * (y1 - y0);
        let b = -2 * (x1 - x0);
        let (mut wide, mut tall) = (false, false);

        let mut d;
        let (mut loop_start, loop_end);
        let (dy_east, dy_northeast, dx_east, dx_northeast, d_east, d_northeast);

        // octants 1/8
        if (x1 - x0).abs() >= (y1 - y0).abs() {
            wide = true;
            loop_start = x;
            loop_end = x1;
            dx_east = 1;
            dx_northeast = 1;
            dy_east = 0;
            d_east = a;
            //octant 1
            if a > 0 {
                d = a + b / 2;
                dy_northeast = 1;
                d_northeast = a + b;
            }
            //octant 8
            else {
                d = a - b / 2;
                dy_northeast = -1;
                d_northeast = a - b;
            }
        }
        // octants 2/7
        else {
            tall = true;
            dx_east = 0;
            dx_northeast = 1;
            //octant 2
            if a > 0 {
                d = a / 2 + b;
                dy_east = 1;
                dy_northeast = 1;
                d_northeast = a + b;
                d_east = b;
                loop_start = y;
                loop_end = y1;
            }
            //octant 7
            else {
                d = a / 2 - b;
                dy_east = -1;
                dy_northeast = -1;
                d_northeast = a - b;
                d_east = -b;
                loop_start = y1;
                loop_end = y;
            }
        }
        // draw points
        while loop_start < loop_end {
            //plot( s, zb, c, x, y, 0);
            self.draw_point(x, y, c);
            if (wide && ((a > 0 && d > 0) || (a < 0 && d < 0)))
                || (tall && ((a > 0 && d < 0) || (a < 0 && d > 0)))
            {
                y += dy_northeast;
                d += d_northeast;
                x += dx_northeast;
            } else {
                x += dx_east;
                y += dy_east;
                d += d_east;
            }
            loop_start += 1;
        }
        self.draw_point(x1, y1, c);
    }

    pub fn draw_lines(&mut self, edges: &Matrix, c: Color) {
        // Iterate over the edge list 2 points at a time
        for edge in edges.m.chunks_exact(2) {
            self.draw_line(
                edge[0][0] as i32,
                edge[0][1] as i32,
                edge[1][0] as i32,
                edge[1][1] as i32,
                c,
            );
        }
    }

    pub fn draw_polygons(&mut self, polygons: &Matrix, c: Color) {
        // Iterate over the edge list 3 points at a time
        for edge in polygons.m.chunks_exact(3) {
            // Get normal vector for backface culling
            let normal = Vector::calculate_normal(edge);

            if normal.z > 0.0 {
                self.draw_line(
                    edge[0][0] as i32,
                    edge[0][1] as i32,
                    edge[1][0] as i32,
                    edge[1][1] as i32,
                    c,
                );
                self.draw_line(
                    edge[0][0] as i32,
                    edge[0][1] as i32,
                    edge[2][0] as i32,
                    edge[2][1] as i32,
                    c,
                );
                self.draw_line(
                    edge[1][0] as i32,
                    edge[1][1] as i32,
                    edge[2][0] as i32,
                    edge[2][1] as i32,
                    c,
                );
            }
        }
    }

    //fn scanline_convert(&mut self,)
}

impl fmt::Display for Screen {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        // Allocate a string with enough space to hold all the pixels
        // Each pixel has 3 rgb values, which each are at most 4 bytes
        // Add in YRES bytes because every row has a newline character
        // and add in 50 bytes as padding to make sure we don't reallocate
        // Total is `XRES * YRES * 3 * 4 + YRES + 50`
        let size: usize = PIXELS * 3 * 4 + YRES + 50;
        let mut contents = String::with_capacity(size);
        for row in self.pixels.iter() {
            for pixel in row.iter() {
                contents.push_str(&pixel.to_string());
            }
            contents.push_str("\n");
        }
        write!(f, "P3 {} {} 255\n{}", XRES, YRES, contents)
    }
}
