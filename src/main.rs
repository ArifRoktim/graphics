use std::error::Error;

mod screen;
use screen::Screen;
use screen::Color;
use screen::Point;

const COLUMNS: usize = 750;
const ROWS: usize = 750;

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

    screen.draw_point(&Point { x: 25, y: 25 }, Color::white())?;
    screen.draw_point(&Point { x: 0, y: 0 }, Color::white())?;
    // Out of bounds
    // Error: OutOfBounds(Point(100, 2000))
    //screen.draw_point(&Point(100, 2000), Color::white())?;
    let somepoint = Point { x: 250, y: 250 };

    // === Test different line types ===
    // vertical line
    screen.draw_line(&somepoint, &Point { x: 250, y: 400 }, Color::cyan())?;
    screen.draw_line(
        &Point { x: 150, y: 500 },
        &Point { x: 150, y: 400 },
        Color::cyan(),
    )?;
    // horizontal line
    screen.draw_line(&somepoint, &Point { x: 400, y: 250 }, Color::purple())?;
    screen.draw_line(
        &Point { x: 400, y: 150 },
        &Point { x: 250, y: 150 },
        Color::purple(),
    )?;
    // octant 1
    screen.draw_line(
        &Point { x: 300, y: 300 },
        &Point { x: 400, y: 350 },
        Color::green(),
    )?;
    screen.draw_line(&somepoint, &Point { x: 500, y: 500 }, Color::green())?;
    // octant 2
    screen.draw_line(
        &Point { x: 300, y: 300 },
        &Point { x: 350, y: 400 },
        Color::yellow(),
    )?;
    screen.draw_line(
        &Point { x: 700, y: 650 },
        &Point { x: 500, y: 400 },
        Color::yellow(),
    )?;
    // octant 8
    screen.draw_line(
        &Point { x: 350, y: 400 },
        &Point { x: 400, y: 350 },
        Color::red(),
    )?;
    screen.draw_line(
        &Point { x: 350, y: 400 },
        &Point { x: 500, y: 375 },
        Color::red(),
    )?;
    // octant 7
    screen.draw_line(&somepoint, &Point { x: 300, y: 150 }, Color::white())?;
    screen.draw_line(&Point { x: 275, y: 100 }, &somepoint, Color::white())?;

    // do a frick ton of tests by drawing a circle
    let center = Point { x: 550, y: 200 };
    let radius: f64 = 150.0;
    for x in (center.x - radius as usize)..(center.x + radius as usize + 1) {
        // (x-h)^2 + (y-k)^2 = r^2
        // ... algebra here ...
        // y = k +- sqrt(r^2 - (x-h)^2 )
        // This is a crap way to draw a circle. Since I only increment x by 1,
        //   the circle has very few lines at 0 and pi radians.
        // But it's good enough for showing the different octants
        let inside = radius.powf(2.0) - (x as f64 - center.x as f64).powf(2.0);

        let other = Point {
            x,
            y: (center.y as f64 + inside.sqrt()) as usize,
        };
        screen.draw_line(&center, &other, color_the_slope(&center, &other))?;
        // self.draw_line(p0, p1) is the same as self.draw_line(p1, p0)
        //screen.draw_line(&other, &center, color_the_slope(&center, &other))?;

        let other = Point {
            x,
            y: (center.y as f64 - inside.sqrt()) as usize,
        };
        screen.draw_line(&center, &other, color_the_slope(&center, &other))?;

        //let y = (center.y as f64 - inside.sqrt()) as usize;
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

    let testing = Color::new(2, 60, 0);
    println!("{}", testing.red);
    Ok(())
}
