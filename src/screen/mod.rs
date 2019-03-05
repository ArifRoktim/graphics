use std::error;
use std::{fs::File, io::prelude::*};
use std::fmt;
use std::path::Path;
use std::process::Command;

mod color;
mod point;
pub use color::Color;
pub use point::Point;
use crate::COLUMNS;
use crate::ROWS;

use crate::Matrix;

pub struct Screen {
    pub pixels: Vec<Vec<Color>>,
}

impl Screen {
    pub fn blank() -> Screen {
        Screen {
            pixels: vec![vec![Color::black(); COLUMNS]; ROWS],
        }
    }

    pub fn new(c: Color) -> Screen {
        Screen {
            pixels: vec![vec![c; COLUMNS]; ROWS],
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
                if let Ok(mut proc) = Command::new("convert")
                    .arg(ppm).arg(f).spawn() {
                        proc.wait().unwrap();
                    } else {
                        eprintln!("Error running `convert` command! \
                                  Is Image Magick installed?");
                    }
            },
            None => panic!("Please specify a file name!"),
        }
        Ok(())
    }

    pub fn fill(&mut self, c: Color) {
        for row in self.pixels.iter_mut() {
            for pixel in row.iter_mut() {
                pixel.color(c)
            }
        }
    }

    pub fn draw_point(&mut self, p: &Point, c: Color) -> Result<(), OutOfBounds> {
        // Make (0, 0) the bottom left corner instead of
        // the top left corner
        if p.x >= COLUMNS || p.y >= ROWS {
            let point = Point { x: p.x, y: p.y };
            return Err(OutOfBounds(point));
        }
        let p = Point {
            x: p.x,
            y: ROWS - 1 - p.y,
        };
        // Get the pixel at point p and set its color
        // Man this looks ugly :(
        &self.pixels[p.y][p.x].color(c);
        Ok(())
    }

    pub fn draw_line(&mut self, p0: &Point, p1: &Point, c: Color) -> Result<(), OutOfBounds> {
        // this draws from left to right, from p0 to p1
        // if p0 is to the right of p1, swap them
        if p0.x > p1.x {
            self.draw_line(p1, p0, c)?;
            return Ok(());
        }
        match p0.slope(&p1) {
            // if slope is undefined/none, the line is vertical
            // self._vertical_line(p0, p1, ...) assumes that
            // p0's y value is less than that of p1.
            // This accounts for that case.
            None if p0.y <= p1.y => self._vertical_line(p0, p1, c)?,
            // This accounts for case where p1's y value > p0's y value
            None => self._vertical_line(p1, p0, c)?,
            Some(m) if m == 0.0 => self._horizontal_line(p0, p1, c)?,
            Some(m) if m > 0.0 && m <= 1.0 => self._octant1(p0, p1, c)?,
            Some(m) if m > 1.0 => self._octant2(p0, p1, c)?,
            Some(m) if m < 0.0 && m >= -1.0 => self._octant8(p0, p1, c)?,
            Some(m) if m < -1.0 => self._octant7(p0, p1, c)?,
            Some(m) => panic!("Slope={}, not yet covered!", m),
        }
        Ok(())
    }

    pub fn draw_lines(&mut self, m: &Matrix, c: Color) -> Result<(), OutOfBounds>{
        // Iterate over the edge list 2 points at a time
        for edge in m.m.chunks_exact(2) {
            let p0 = Point{ x: edge[0][0] as usize, y: edge[0][1] as usize};
            let p1 = Point{ x: edge[1][0] as usize, y: edge[1][1] as usize};
            self.draw_line(&p0, &p1, c)?
        }
        Ok(())
    }

    // ========== PRIVATE FUNCTIONS START ==========
    fn _vertical_line(&mut self, p0: &Point, p1: &Point, c: Color) -> Result<(), OutOfBounds> {
        for i in p0.y..p1.y {
            self.draw_point(&Point { x: p0.x, y: i }, c)?;
        }
        Ok(())
    }
    fn _horizontal_line(&mut self, p0: &Point, p1: &Point, c: Color) -> Result<(), OutOfBounds> {
        for i in p0.x..p1.x {
            self.draw_point(&Point { x: i, y: p0.y }, c)?;
        }
        Ok(())
    }
    fn _octant1(&mut self, p0: &Point, p1: &Point, c: Color) -> Result<(), OutOfBounds> {
        // First cast the points to i32 from usize
        let p0 = (p0.x as i32, p0.y as i32);
        let p1 = (p1.x as i32, p1.y as i32);
        // x and y points to plot
        let mut x = p0.0;
        let mut y = p0.1;
        // B = - delta_x
        let delta_x = p1.0 - p0.0;
        // A = delta_y
        let delta_y = p1.1 - p0.1;
        // d = f(x0 + 1, y0 + 1/2) - f(x0, y0)
        // f(x0, y0) = 0
        // d = f(x0 + 1, y0 + 1/2)
        // ... <Algebra goes here> ...
        // d = delta_y - 1/2 * delta_x
        // To get rid of floating point arithmetic, multiply by 2
        // 2d = 2 * delta_y - delta_x
        let mut diff = 2 * delta_y - delta_x;
        while x <= p1.0 {
            self.draw_point(
                &Point {
                    x: x as usize,
                    y: y as usize,
                },
                c,
            )?;
            if diff > 0 {
                y += 1;
                diff -= 2 * delta_x;
            }
            x += 1;
            diff += 2 * delta_y;
        }
        Ok(())
    }
    fn _octant2(&mut self, p0: &Point, p1: &Point, c: Color) -> Result<(), OutOfBounds> {
        // First cast the points to i32 from usize
        let p0 = (p0.x as i32, p0.y as i32);
        let p1 = (p1.x as i32, p1.y as i32);
        // x and y points to plot
        let mut x = p0.0;
        let mut y = p0.1;
        // B = - delta_x
        let delta_x = p1.0 - p0.0;
        // A = delta_y
        let delta_y = p1.1 - p0.1;
        // d = f(x0 + 1/2, y0 + 1)
        // ... <Algebra goes here> ...
        // d = 1/2 * delta_y - delta_x
        // 2d = delta_y - 2 * delta_x
        let mut diff = delta_y - 2 * delta_x;
        while y <= p1.1 {
            self.draw_point(
                &Point {
                    x: x as usize,
                    y: y as usize,
                },
                c,
            )?;
            if diff < 0 {
                x += 1;
                diff += 2 * delta_y;
            }
            y += 1;
            diff -= 2 * delta_x;
        }
        Ok(())
    }
    fn _octant8(&mut self, p0: &Point, p1: &Point, c: Color) -> Result<(), OutOfBounds> {
        // First cast the points to i32 from usize
        let p0 = (p0.x as i32, p0.y as i32);
        let p1 = (p1.x as i32, p1.y as i32);
        // x and y points to plot
        let mut x = p0.0;
        let mut y = p0.1;
        // B = - delta_x
        let delta_x = p1.0 - p0.0;
        // A = delta_y
        let delta_y = p1.1 - p0.1;
        // d = f(x0 + 1, y0 - 1/2)
        // d = A(x0 + 1) + B(y0 - 1/2) + C
        // d = (Ax0 + By0 + C) + A - 1/2 * B
        // d = A - 1/2 * B
        // d = delta_y + 1/2 * delta_x
        // 2d = 2 * delta_y + delta_x
        let mut diff = 2 * delta_y + delta_x;
        while x <= p1.0 {
            self.draw_point(
                &Point {
                    x: x as usize,
                    y: y as usize,
                },
                c,
            )?;
            if diff > 0 {
                y -= 1;
                diff -= 2 * delta_x;
            }
            x += 1;
            diff -= 2 * delta_y;
        }
        Ok(())
    }
    fn _octant7(&mut self, p0: &Point, p1: &Point, c: Color) -> Result<(), OutOfBounds> {
        // First cast the points to i32 from usize
        let p0 = (p0.x as i32, p0.y as i32);
        let p1 = (p1.x as i32, p1.y as i32);
        // x and y points to plot
        let mut x = p0.0;
        let mut y = p0.1;
        // B = - delta_x
        let delta_x = p1.0 - p0.0;
        // A = delta_y
        let delta_y = p1.1 - p0.1;
        // d = f(x0 + 1/2, y0 - 1)
        // d = A(x0 + 1/2) + B(y0 - 1) + C
        // d = (Ax0 + By0 + C) + 1/2 * A - B
        // d = 1/2 * A - B
        // d = 1/2 * delta_y - (- delta_x) = 1/2 * delta_y + delta_x
        // 2d = delta_y + 2 * delta_x
        let mut diff = delta_y + 2 * delta_x;
        //let mut diff = -2 * delta_x - delta_y;
        while y >= p1.1 {
            self.draw_point(
                &Point {
                    x: x as usize,
                    y: y as usize,
                },
                c,
            )?;
            if diff < 0 {
                x += 1;
                diff -= 2 * delta_y;
            }
            y -= 1;
            diff -= 2 * delta_x;
        }
        Ok(())
    }
    // ========== PRIVATE FUNCTIONS END ==========
}

impl fmt::Display for Screen {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        // Allocate a string with enough space to hold all the pixels
        // COLUMNS * ROWS gives the number of pixels
        // Each pixel has 3 rgb values, which each are at most 4 bytes
        // Add in ROWS bytes because every row has a newline character
        // and add in 50 bytes as padding to make sure we don't reallocate
        // Total is `COLUMNS * ROWS * 3 * 4 + ROWS + 50`
        let size: usize = COLUMNS * ROWS * 3 * 4 + ROWS + 50;
        let mut contents = String::with_capacity(size);
        for row in &self.pixels {
            for pixel in row {
                contents.push_str(&pixel.to_string());
            }
            contents.push_str("\n");
        }
        write!(f, "P3 {} {} 255\n{}", COLUMNS, ROWS, contents)
    }
}

#[derive(Debug)]
pub struct OutOfBounds(pub Point);

impl fmt::Display for OutOfBounds {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Point ({}) is out of bounds", self.0)
    }
}

// This is important for other errors to wrap this one.
impl error::Error for OutOfBounds {
    fn cause(&self) -> Option<&error::Error> {
        // Generic error, underlying cause isn't tracked.
        None
    }
}
