use std::error;
use std::error::Error;
use std::fmt;
use std::fs::File;
use std::io::prelude::*;

const COLUMNS: usize = 750;
const ROWS: usize = 750;

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

impl fmt::Display for Point {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}, {}", self.0, self.1)
    }
}

#[derive(Debug)]
struct OutOfBounds(Point);

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

struct Screen {
    pixels: Vec<Vec<Color>>,
}

impl Screen {
    fn blank() -> Screen {
        Screen {
            pixels: vec![vec![Color::black(); COLUMNS]; ROWS],
        }
    }

    fn new(c: Color) -> Screen {
        Screen {
            pixels: vec![vec![c; COLUMNS]; ROWS],
        }
    }

    fn write(&self, f: &str) -> std::io::Result<()> {
        let mut file = File::create(f)?;
        file.write_all(self.to_string().as_bytes())
    }

    // TODO: These should probably return a Result.
    // Maybe make an OutofBounds error?
    //fn draw_point(&mut self, p: &Point, c: Color) {
    fn draw_point(&mut self, p: &Point, c: Color) -> Result<(), OutOfBounds> {
        // Make (0, 0) the bottom left corner instead of
        // the top left corner
        if p.0 >= COLUMNS || p.1 >= ROWS {
            let point = Point(p.0, p.1);
            return Err(OutOfBounds(point));
        }
        let p = Point(p.0, ROWS - 1 - p.1);
        // Get the pixel ay point p and set its color
        // Man this looks ugly :(
        &self.pixels[p.1][p.0].color(c);
        Ok(())
    }

    fn draw_line(&mut self, p0: &Point, p1: &Point, c: Color) -> Result<(), OutOfBounds> {
        // this draws from left to right, from p0 to p1
        // if p0 is to the right of p1, swap them
        if p0.0 > p1.0 {
            self.draw_line(p1, p0, c)?;
            return Ok(());
        }
        match p0.slope(&p1) {
            None => println!("{:?}, {:?} has slope=undefined", p0, p1),
            Some(m) => println!("{:?}, {:?} has slope={}", p0, p1, m),
        }
        match p0.slope(&p1) {
            // if slope is undefined/none, line is vertical
            None => self._vertical_line(p0, p1, c)?,
            Some(m) if m == 0.0 => self._horizontal_line(p0, p1, c)?,
            Some(m) if m > 0.0 && m <= 1.0 => self._octant1(p0, p1, c)?,
            Some(m) if m > 1.0 => self._octant2(p0, p1, c)?,
            Some(m) => panic!("Slope={}, not yet covered!", m),
        }
        Ok(())
    }

    //fn _octant1(&mut self, p0: &Point, p1: &Point, c: Color) {}
    fn _vertical_line(&mut self, p0: &Point, p1: &Point, c: Color) -> Result<(), OutOfBounds> {
        for i in p0.1..p1.1 {
            self.draw_point(&Point(p0.0, i), c)?;
        }
        Ok(())
    }
    fn _horizontal_line(&mut self, p0: &Point, p1: &Point, c: Color) -> Result<(), OutOfBounds> {
        for i in p0.0..p1.0 {
            self.draw_point(&Point(i, p0.1), c)?;
        }
        Ok(())
    }
    fn _octant1(&mut self, p0: &Point, p1: &Point, c: Color) -> Result<(), OutOfBounds> {
        // First cast the points to i32 from usize
        let p0 = (p0.0 as i32, p0.1 as i32);
        let p1 = (p1.0 as i32, p1.1 as i32);
        // x and y points to plot
        let mut x = p0.0;
        let mut y = p0.1;
        let delta_x = p1.0 - p0.0;
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
            self.draw_point(&Point(x as usize, y as usize), c)?;
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
        let p0 = (p0.0 as i32, p0.1 as i32);
        let p1 = (p1.0 as i32, p1.1 as i32);
        // x and y points to plot
        let mut x = p0.0;
        let mut y = p0.1;
        let delta_x = p1.0 - p0.0;
        let delta_y = p1.1 - p0.1;
        // d = f(x0 + 1/2, y0 + 1)
        // ... <Algebra goes here> ...
        // d = 1/2 * delta_y - delta_x
        // 2d = delta_y - 2 * delta_x
        let mut diff = delta_y - 2 * delta_x;
        while y <= p1.1 {
            self.draw_point(&Point(x as usize, y as usize), c)?;
            if diff < 0 {
                x += 1;
                diff += 2 * delta_y;
            }
            y += 1;
            diff -= 2 * delta_x;
        }
        Ok(())
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

fn main() -> Result<(), Box<dyn Error>> {
    let mut screen = Screen::blank();
    //let mut screen = Screen::new(Color::white());

    screen.draw_point(&Point(25, 25), Color::white())?;
    screen.draw_point(&Point(0, 0), Color::white())?;
    // Out of bounds
    // Error: OutOfBounds(Point(100, 2000))
    //screen.draw_point(&Point(100, 2000), Color::white())?;
    let somepoint = Point(250, 250);

    // === Test different line types ===
    // vertical line
    screen.draw_line(&somepoint, &Point(250, 400), Color::cyan())?;
    // horizontal line
    screen.draw_line(&somepoint, &Point(400, 250), Color::purple())?;
    screen.draw_line(&Point(400, 150), &Point(250, 150), Color::purple())?;
    // octant 1
    screen.draw_line(&Point(300, 300), &Point(400, 350), Color::green())?;
    screen.draw_line(&somepoint, &Point(499, 499), Color::green())?;
    // octant 2
    screen.draw_line(&Point(300, 300), &Point(350, 400), Color::yellow())?;

    screen.write("out.ppm").expect("Failed to write to file!");

    Ok(())
}
