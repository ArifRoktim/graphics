use crate::matrix::Matrix;
use crate::vector::Vector;
use crate::XRES;
use crate::YRES;
use std::fmt;
use std::path::Path;
use std::process::Command;
use std::{fs::File, io::prelude::*};

pub mod color;
pub mod line;

pub use color::Color;
pub use line::Line;

pub struct Screen {
    pub pixels: Vec<Vec<Color>>,
    pub color: Color,
}

impl Screen {
    pub fn blank() -> Screen {
        Screen {
            pixels: vec![vec![color::BLACK; XRES]; YRES],
            color: color::BLACK,
        }
    }

    pub fn new(c: Color) -> Screen {
        Screen {
            pixels: vec![vec![c; XRES]; YRES],
            color: c,
        }
    }

    pub fn write_ppm(&self, f: &str) -> std::io::Result<()> {
        let mut file = File::create(f)?;
        file.write_all(self.to_string().as_bytes())
    }

    pub fn write(&self, f: &str) -> std::io::Result<()> {
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
            }
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
        let (px, py): (usize, usize) = (px as usize, YRES - 1 - (py as usize));
        // Get the pixel at point p and set its color
        self.pixels[py][px].color(c);
    }

    pub fn draw_line(&mut self, p0x: i32, p0y: i32, p1x: i32, p1y: i32, c: Color) {
        match Line::get_octant(p0x, p0y, p1x, p1y) {
            (Line::Horizontal, line) => self._horizontal_line(line, c),
            (Line::Vertical, line) => self._vertical_line(line, c),
            (Line::Octant1, line) => self._octant1(line, c),
            (Line::Octant2, line) => self._octant2(line, c),
            (Line::Octant7, line) => self._octant7(line, c),
            (Line::Octant8, line) => self._octant8(line, c),
        }
    }

    pub fn draw_lines(&mut self, m: &Matrix, c: Color) {
        // Iterate over the edge list 2 points at a time
        for edge in m.m.chunks_exact(2) {
            // If any of the points have negative coords, they can't be drawn
            //if edge[0][0] < 0.0 || edge[0][1] < 0.0 ||
            //    edge[1][0] < 0.0 || edge[1][1] < 0.0 {
            //        continue;
            //}
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
}

// private functions
impl Screen {
    fn _vertical_line(&mut self, line: (i32, i32, i32, i32), c: Color) {
        let (p0x, p0y, _, p1y) = line;
        for y in p0y..=p1y {
            self.draw_point(p0x, y, c);
        }
    }
    fn _horizontal_line(&mut self, line: (i32, i32, i32, i32), c: Color) {
        let (p0x, p0y, p1x, _) = line;
        for x in p0x..=p1x {
            self.draw_point(x, p0y, c);
        }
    }
    fn _octant1(&mut self, line: (i32, i32, i32, i32), c: Color) {
        let (p0x, p0y, p1x, p1y) = line;
        // x and y points to plot
        let mut x = p0x;
        let mut y = p0y;
        // B = - delta_x
        let delta_x = p1x - p0x;
        // A = delta_y
        let delta_y = p1y - p0y;
        // d = f(x0 + 1, y0 + 1/2) - f(x0, y0)
        // f(x0, y0) = 0
        // d = f(x0 + 1, y0 + 1/2)
        // ... <Algebra goes here> ...
        // d = delta_y - 1/2 * delta_x
        // To get rid of floating point arithmetic, multiply by 2
        // 2d = 2 * delta_y - delta_x
        let mut diff = 2 * delta_y - delta_x;
        while x <= p1x {
            self.draw_point(x, y, c);
            if diff > 0 {
                y += 1;
                diff -= 2 * delta_x;
            }
            x += 1;
            diff += 2 * delta_y;
        }
    }
    fn _octant2(&mut self, line: (i32, i32, i32, i32), c: Color) {
        let (p0x, p0y, p1x, p1y) = line;
        // First cast the points to i32 from usize
        // x and y points to plot
        let mut x = p0x;
        let mut y = p0y;
        // B = - delta_x
        let delta_x = p1x - p0x;
        // A = delta_y
        let delta_y = p1y - p0y;
        // d = f(x0 + 1/2, y0 + 1)
        // ... <Algebra goes here> ...
        // d = 1/2 * delta_y - delta_x
        // 2d = delta_y - 2 * delta_x
        let mut diff = delta_y - 2 * delta_x;
        while y <= p1y {
            self.draw_point(x, y, c);
            if diff < 0 {
                x += 1;
                diff += 2 * delta_y;
            }
            y += 1;
            diff -= 2 * delta_x;
        }
    }
    fn _octant8(&mut self, line: (i32, i32, i32, i32), c: Color) {
        let (p0x, p0y, p1x, p1y) = line;
        // First cast the points to i32 from usize
        // x and y points to plot
        let mut x = p0x;
        let mut y = p0y;
        // B = - delta_x
        let delta_x = p1x - p0x;
        // A = delta_y
        let delta_y = p1y - p0y;
        // d = f(x0 + 1, y0 - 1/2)
        // d = A(x0 + 1) + B(y0 - 1/2) + C
        // d = (Ax0 + By0 + C) + A - 1/2 * B
        // d = A - 1/2 * B
        // d = delta_y + 1/2 * delta_x
        // 2d = 2 * delta_y + delta_x
        let mut diff = 2 * delta_y + delta_x;
        while x <= p1x {
            self.draw_point(x, y, c);
            if diff > 0 {
                y -= 1;
                diff -= 2 * delta_x;
            }
            x += 1;
            diff -= 2 * delta_y;
        }
    }
    fn _octant7(&mut self, line: (i32, i32, i32, i32), c: Color) {
        let (p0x, p0y, p1x, p1y) = line;
        // First cast the points to i32 from usize
        // x and y points to plot
        let mut x = p0x;
        let mut y = p0y;
        // B = - delta_x
        let delta_x = p1x - p0x;
        // A = delta_y
        let delta_y = p1y - p0y;
        // d = f(x0 + 1/2, y0 - 1)
        // d = A(x0 + 1/2) + B(y0 - 1) + C
        // d = (Ax0 + By0 + C) + 1/2 * A - B
        // d = 1/2 * A - B
        // d = 1/2 * delta_y - (- delta_x) = 1/2 * delta_y + delta_x
        // 2d = delta_y + 2 * delta_x
        let mut diff = delta_y + 2 * delta_x;
        //let mut diff = -2 * delta_x - delta_y;
        while y >= p1y {
            self.draw_point(x, y, c);
            if diff < 0 {
                x += 1;
                diff -= 2 * delta_y;
            }
            y -= 1;
            diff -= 2 * delta_x;
        }
    }
}

impl fmt::Display for Screen {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        // Allocate a string with enough space to hold all the pixels
        // XRES * YRES gives the number of pixels
        // Each pixel has 3 rgb values, which each are at most 4 bytes
        // Add in YRES bytes because every row has a newline character
        // and add in 50 bytes as padding to make sure we don't reallocate
        // Total is `XRES * YRES * 3 * 4 + YRES + 50`
        let size: usize = XRES * YRES * 3 * 4 + YRES + 50;
        let mut contents = String::with_capacity(size);
        for row in &self.pixels {
            for pixel in row {
                contents.push_str(&pixel.to_string());
            }
            contents.push_str("\n");
        }
        write!(f, "P3 {} {} 255\n{}", XRES, YRES, contents)
    }
}
