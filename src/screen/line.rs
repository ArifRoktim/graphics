use std::cmp;

pub enum Line {
    Horizontal,
    Vertical,
    Octant1,
    Octant2,
    Octant7,
    Octant8,
}

impl Line {
    pub fn get_octant(p0x: i32, p0y: i32, p1x: i32, p1y: i32) -> (Line, (i32, i32, i32, i32)) {
        // line is vertical
        if p0x == p1x {
            // make sure we draw from the lower y value to the higher one
            let (p0y, p1y) = (cmp::min(p0y, p1y), cmp::max(p0y, p1y));
            (Line::Vertical, (p0x, p0y, p1x, p1y))
        }
        // make sure we draw from left to right on the screen
        else if p0x > p1x {
            Line::get_octant(p1x, p1y, p0x, p0y)
        }
        else if p0y == p1y {
            (Line::Horizontal, (p0x, p0y, p1x, p1y))
        }
        // Magnitude of slope is > 1 so line is in octant 1 or 8
        else if (p1x - p0x).abs() >= (p1y - p0y).abs() {
            if p1y - p0y > 0 {
                (Line::Octant1, (p0x, p0y, p1x, p1y))
            } else {
                (Line::Octant8, (p0x, p0y, p1x, p1y))
            }
        }
        // Magnitude of slope is between 0 and 1 so line is in octant 2 or 7
        else if p1y - p0y > 0 {
            (Line::Octant2, (p0x, p0y, p1x, p1y))
        } else {
            (Line::Octant7, (p0x, p0y, p1x, p1y))
        }
    }
}
