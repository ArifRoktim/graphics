use std::fmt;
use std::fs::File;
use std::io::prelude::*;

const COLUMNS: usize = 500;
const ROWS: usize = 500;

#[derive(Debug, Copy, Clone)]
struct Color {
    red: u8,
    green: u8,
    blue: u8,
}

#[allow(dead_code)]
impl Color {
    fn new(r: u8, g: u8, b: u8) -> Color {
        Color {
            red: r,
            green: g,
            blue: b,
        }
    }

    fn black() -> Color {
        Color {
            red: 0,
            green: 0,
            blue: 0,
        }
    }

    fn white() -> Color {
        Color {
            red: 255,
            green: 255,
            blue: 255,
        }
    }

    fn red() -> Color {
        Color {
            red: 255,
            green: 0,
            blue: 0,
        }
    }

    fn green() -> Color {
        Color {
            red: 0,
            green: 255,
            blue: 0,
        }
    }

    fn blue() -> Color {
        Color {
            red: 0,
            green: 0,
            blue: 255,
        }
    }

    fn yellow() -> Color {
        Color {
            red: 255,
            green: 255,
            blue: 0,
        }
    }

    fn purple() -> Color {
        Color {
            red: 255,
            green: 0,
            blue: 255,
        }
    }

    fn cyan() -> Color {
        Color {
            red: 0,
            green: 255,
            blue: 255,
        }
    }

    fn color(&mut self, c: Color) {
        self.red = c.red;
        self.green = c.green;
        self.blue = c.blue;
    }
}

impl fmt::Display for Color {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{} {} {} ", self.red, self.green, self.blue)
    }
}

// Point(x, y)
struct Point(usize, usize);

impl Point {
    // If undefined, return None, otherwise return Some(i32)
    fn slope(&self, other: Point) -> Option<i32> {
        let me = (self.0 as i32, self.1 as i32);
        let other = (other.0 as i32, other.1 as i32);
        if me.0 - other.0 == 0 {
            None
        } else {
            Some((me.1 - other.1) / (me.0 - other.0))
        }
    }

}

struct Screen {
    pixels: Vec<Vec<Color>>,
}

impl Screen {
    fn new() -> Screen {
        Screen {
            pixels: vec![vec![Color::black(); COLUMNS]; ROWS],
        }
    }

    fn write(&self, f: &str) -> std::io::Result<()> {
        let mut file = File::create(f)?;
        file.write_all(self.to_string().as_bytes())
    }

    // TODO: These should probably return a Result.
    // Maybe make an OutofBounds error?
    fn draw_point(&mut self, p: Point, c: Color) {
        // Make (0, 0) the bottom left corner instead of
        // the top left corner
        let p = Point(p.0, ROWS - 1 - p.1);
        // Get the pixel ay point p and set its color
        // Man this looks ugly :(
        &self.pixels[p.1][p.0].color(c);
    }

    //fn draw_line(&mut self, p0: Point, p1: Point, c: Color) {
    //    //
    //}

    //fn _octant1(&mut self, p0: Point, p1: Point, c: Color) {}
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

fn main() {
    //let p1 = Point(0, 10);
    //let p2 = Point(10, 10);
    //match p1.slope(p2) {
    //    Some(x) => println!("{}", x),
    //    None    => println!("undefined"),
    //}

    let mut screen = Screen::new();

    screen.draw_point(Point(25, 25), Color::new(255, 255, 255));

    screen.write("out.ppm").expect("Failed to write to file!");

}
