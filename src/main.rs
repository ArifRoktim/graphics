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
            // if slope is undefined/none, the line is vertical
            // self._vertical_line(p0, p1, ...) assumes that
            // p0's y value is less than that of p1.
            // This accounts for that case.
            None if p0.1 <= p1.1 => self._vertical_line(p0, p1, c)?,
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

    // ========== HELPER FUNCTIONS START ==========
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
    fn _octant8(&mut self, p0: &Point, p1: &Point, c: Color) -> Result<(), OutOfBounds> {
        // First cast the points to i32 from usize
        let p0 = (p0.0 as i32, p0.1 as i32);
        let p1 = (p1.0 as i32, p1.1 as i32);
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
            self.draw_point(&Point(x as usize, y as usize), c)?;
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
        let p0 = (p0.0 as i32, p0.1 as i32);
        let p1 = (p1.0 as i32, p1.1 as i32);
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
            self.draw_point(&Point(x as usize, y as usize), c)?;
            if diff < 0 {
                x += 1;
                diff -= 2 * delta_y;
            }
            y -= 1;
            diff -= 2 * delta_x;
        }
        Ok(())
    }
    // ========== HELPER FUNCTIONS END ==========
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

fn color_the_slope(p0: &Point, p1: &Point) -> Color {
    match p0.slope(&p1) {
        None => Color::cyan(),
        Some(m) if m == 0.0 => Color::purple(),
        Some(m) if m > 0.0 && m <= 1.0 => Color::green(),
        Some(m) if m > 1.0 => Color::yellow(),
        Some(m) if m < 0.0 && m >= -1.0 => Color::red(),
        Some(m) if m < -1.0 => Color::white(),
        Some(m) => panic!("Slope={}, not yet covered!", m),
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
    screen.draw_line(&Point(150, 500), &Point(150, 400), Color::cyan())?;
    // horizontal line
    screen.draw_line(&somepoint, &Point(400, 250), Color::purple())?;
    screen.draw_line(&Point(400, 150), &Point(250, 150), Color::purple())?;
    // octant 1
    screen.draw_line(&Point(300, 300), &Point(400, 350), Color::green())?;
    screen.draw_line(&somepoint, &Point(500, 500), Color::green())?;
    // octant 2
    screen.draw_line(&Point(300, 300), &Point(350, 400), Color::yellow())?;
    screen.draw_line(&Point(700, 650), &Point(500, 400), Color::yellow())?;
    // octant 8
    screen.draw_line(&Point(350, 400), &Point(400, 350), Color::red())?;
    screen.draw_line(&Point(350, 400), &Point(500, 375), Color::red())?;
    // octant 7
    screen.draw_line(&somepoint, &Point(300, 150), Color::white())?;
    screen.draw_line(&Point(275, 100), &somepoint, Color::white())?;

    // do a frick ton of tests by drawing a circle
    let center = Point(550, 200);
    let radius: f64 = 150.0;
    for x in (center.0 - radius as usize)..(center.0 + radius as usize + 1) {
        // (x-h)^2 + (y-k)^2 = r^2
        // ... algebra here ...
        // y = k +- sqrt(r^2 - (x-h)^2 )
        // This is a crap way to draw a circle. Since I only increment x by 1,
        //   the circle has very few lines at 0 and pi radians.
        // But it's good enough for showing the different octants
        let inside = radius.powf(2.0)  - (x as f64 - center.0 as f64).powf(2.0);

        let other = Point(x, (center.1 as f64 + inside.sqrt()) as usize);
        screen.draw_line(&center, &other, color_the_slope(&center, &other))?;
        // self.draw_line(p0, p1) is the same as self.draw_line(p1, p0)
        //screen.draw_line(&other, &center, color_the_slope(&center, &other))?;

        let other = Point(x, (center.1 as f64 - inside.sqrt()) as usize);
        screen.draw_line(&center, &other, color_the_slope(&center, &other))?;

        //let y = (center.1 as f64 - inside.sqrt()) as usize;
        //screen.draw_line(&center, &Point(x, y), Color::green())?;
    }

    screen.write("out.ppm").expect("Failed to write to file!");

    let info = "Vertical lines are cyan.\n\
        Horizontal lines are purple.\n\
        Octant 1 lines are green.\n\
        Octant 2 lines are yellow.\n\
        Octant 8 lines are red.\n\
        Octant 7 lines are white.";
    println!("{}", info);

    Ok(())
}
