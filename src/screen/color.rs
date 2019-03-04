use std::fmt;

#[derive(Debug, Copy, Clone)]
pub struct Color {
    pub red: u8,
    pub green: u8,
    pub blue: u8,
}

impl Color {
    pub fn new(r: u8, g: u8, b: u8) -> Color {
        Color {
            red: r,
            green: g,
            blue: b,
        }
    }

    pub fn black() -> Color {
        Color {
            red: 0,
            green: 0,
            blue: 0,
        }
    }

    pub fn white() -> Color {
        Color {
            red: 255,
            green: 255,
            blue: 255,
        }
    }

    pub fn red() -> Color {
        Color {
            red: 255,
            green: 0,
            blue: 0,
        }
    }

    pub fn green() -> Color {
        Color {
            red: 0,
            green: 255,
            blue: 0,
        }
    }

    pub fn blue() -> Color {
        Color {
            red: 0,
            green: 0,
            blue: 255,
        }
    }

    pub fn yellow() -> Color {
        Color {
            red: 255,
            green: 255,
            blue: 0,
        }
    }

    pub fn purple() -> Color {
        Color {
            red: 255,
            green: 0,
            blue: 255,
        }
    }

    pub fn cyan() -> Color {
        Color {
            red: 0,
            green: 255,
            blue: 255,
        }
    }

    pub fn color(&mut self, c: Color) {
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
