use std::fmt;
use std::fs::File;
use std::io::prelude::*;

const COLUMNS: usize = 500;
const ROWS: usize = 500;

#[derive(Debug, Copy, Clone)]
struct Pixel {
    red: u8,
    green: u8,
    blue: u8,
}

impl Pixel {
    fn new(r: u8, g: u8, b: u8) -> Pixel {
        Pixel {
            red: r,
            green: g,
            blue: b,
        }
    }

    fn black() -> Pixel {
        Pixel {
            red: 0,
            green: 0,
            blue: 0,
        }
    }

    fn white() -> Pixel {
        Pixel {
            red: 255,
            green: 255,
            blue: 255,
        }
    }

    fn color(&mut self, r: u8, g: u8, b: u8) {
        self.red = r;
        self.green = g;
        self.blue = b;
    }
}

impl fmt::Display for Pixel {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{} {} {} ", self.red, self.green, self.blue)
    }
}

struct Screen {
    pixels: Vec<Vec<Pixel>>,
}

impl Screen {
    fn new() -> Screen {
        Screen {
            pixels: vec![vec![Pixel::black(); COLUMNS]; ROWS],
        }
    }

    fn write(&self, f: &str) -> std::io::Result<()> {
        let mut file = File::create(f)?;
        file.write_all(self.to_string().as_bytes())
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

    for (i, row) in screen.pixels.iter_mut().enumerate() {
        for (j, pixel) in row.iter_mut().enumerate() {
            pixel.color(j as u8, i as u8, i as u8);
        }
    }

    screen.write("out.ppm").expect("Failed to write to file!");
}
