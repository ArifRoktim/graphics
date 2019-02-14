use std::fmt;

#[derive(Debug)]
pub struct Point {
    pub x: usize,
    pub y: usize,
}

impl Point {
    // If undefined, return None, otherwise return Some(f64)
    pub fn slope(&self, other: &Point) -> Option<f64> {
        let me = (self.x as f64, self.y as f64);
        let other = (other.x as f64, other.y as f64);
        if me.0 - other.0 == 0.0 {
            None
        } else {
            Some((me.1 - other.1) / (me.0 - other.0))
        }
    }
}

impl fmt::Display for Point {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}, {}", self.x, self.y)
    }
}
