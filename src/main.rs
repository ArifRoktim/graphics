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
#[derive(Debug)]
struct Point(usize, usize);

impl Point {
    // If undefined, return None, otherwise return Some(f64)
    fn slope(&self, other: &Point) -> Option<f64> {
        let me = (self.0 as f64, self.1 as f64);
        let other = (other.0 as f64, other.1 as f64);
        if me.0 - other.0 == 0.0 {
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
    fn draw_point(&mut self, p: &Point, c: Color) {
        // Make (0, 0) the bottom left corner instead of
        // the top left corner
        let p = Point(p.0, ROWS - 1 - p.1);
        // Get the pixel ay point p and set its color
        // Man this looks ugly :(
        &self.pixels[p.1][p.0].color(c);
    }

    fn draw_line(&mut self, p0: &Point, p1: &Point, c: Color) {
        // this draws from left to right, from p0 to p1
        // if p0 is to the right of p1, swap them
        if p0.0 > p1.0 {
            self.draw_line(p1, p0, c);
            return;
        }
        match p0.slope(&p1) {
            None => println!("{:?}, {:?} has slope=undefined", p0, p1),
            Some(m) => println!("{:?}, {:?} has slope={}", p0, p1, m),
        }
        match p0.slope(&p1) {
            // slope is undefined
            None => self._vertical_line(p0, p1, c),
            Some(m) if m == 0.0 => self._horizontal_line(p0, p1, c),
            Some(m) if m > 0.0 && m <= 1.0 => self._octant1(p0, p1, c),
            Some(m) => panic!("Slope={}, not yet covered!", m),
        }
    }

    //fn _octant1(&mut self, p0: &Point, p1: &Point, c: Color) {}
    fn _vertical_line(&mut self, p0: &Point, p1: &Point, c: Color) {
        for i in p0.1..p1.1 {
            self.draw_point(&Point(p0.0, i), c);
        }
    }
    fn _horizontal_line(&mut self, p0: &Point, p1: &Point, c: Color) {
        for i in p0.0..p1.0 {
            self.draw_point(&Point(i, p0.1), c);
        }
    }
    fn _octant1(&mut self, p0: &Point, p1: &Point, c: Color) {
        // First cast the points to i32 from usize
        let p0 = (p0.0 as i32, p0.1 as i32);
        let p1 = (p1.0 as i32, p1.1 as i32);
        // x and y points to plot
        let mut x = p0.0;
        let mut y = p0.1;
        // TODO: explain meaning of A, B, and d
        let A = p1.1 - p0.1;
        let B = -(p1.0 - p0.0);
        let mut d = 2 * A + B;
        while x <= p1.0 {
            self.draw_point(&Point(x as usize, y as usize), c);
            if d > 0 {
                y += 1;
                d += 2 * B;
            }
            x += 1;
            d += 2 * A;
        }
    }
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
    let mut screen = Screen::new();

    screen.draw_point(&Point(25, 25), Color::white());
    let origin = Point(250, 250);

    // === Test different line types ===
    // vertical line
    screen.draw_line(&origin, &Point(250, 400), Color::cyan());
    // horizontal line
    screen.draw_line(&origin, &Point(400, 250), Color::purple());
    screen.draw_line(&Point(400, 150), &Point(250,150), Color::purple());
    // octant 1
    screen.draw_line(&Point(300, 300), &Point(400, 350), Color::green());
    screen.draw_line(&origin, &Point(500, 500), Color::green());

    screen.write("out.ppm").expect("Failed to write to file!");

}
