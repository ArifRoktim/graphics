use crate::matrix::{Matrix, COLS};
use crate::{Light, Vector};
use crate::PICTURE_DIR;
use std::f64;
use std::fmt;
use std::fs::{self, DirBuilder, File};
use std::io::{self, prelude::*};
use std::mem;
use std::ops::{Index, IndexMut};
use std::path::{Path, PathBuf};
use std::process::{Command, Stdio};
use std::slice::{ChunksExact, ChunksExactMut};

pub mod color;
pub use color::{consts, Color, Reflection, Shine};

type Pixel = (Color, f64);

pub struct Screen {
    // Screen properties
    pub pixels: Vec<Pixel>,
    pub xres: usize,
    pub yres: usize,
    pub screen_color: Color,
    pub line_color: Color,
    pub steps_2d: usize,
    pub steps_3d: usize,
    pub specular_exp: i32,
    pub view_vector: Vector,
    pub ambient_light: Color,
    // These are default values if light(s) or reflection(s) aren't provided
    lights: Vec<Light>,
    reflection: Reflection,
}

impl Screen {
    pub fn new(c: Color) -> Screen {
        let mut builder = ScreenBuilder::default();
        builder.screen_color = c;
        builder.create()
    }

    pub fn xres(&self) -> usize {
        self.xres
    }

    pub fn yres(&self) -> usize {
        self.yres
    }

    pub fn rows(&self) -> ChunksExact<Pixel> {
        self.pixels.chunks_exact(self.xres)
    }

    pub fn rows_mut(&mut self) -> ChunksExactMut<Pixel> {
        self.pixels.chunks_exact_mut(self.xres)
    }

    fn write_ppm(&self, f: &Path) -> io::Result<()> {
        let mut file = File::create(f)?;
        file.write_all(self.to_string().as_bytes())
    }

    pub fn write(&self, f: &[&str]) -> io::Result<()> {
        let mut path = PathBuf::from(PICTURE_DIR);

        // Separate f into the file name and its parent(s)
        let last = f.len() - 1;
        let parents = &f[..last];
        let file_name = f[last];
        for parent in parents {
            path.push(parent);
        }

        // Make sure that output directory exists. Create it if not.
        DirBuilder::new().recursive(true).create(&path).unwrap_or_else(|e| {
            panic!("Failed to create image output directory: `{}/`\nError: {}", PICTURE_DIR, e)
        });

        // Add file name to path
        path.push(file_name);

        // Save a copy of the original extension before we replace it
        let extension =
            path.extension().map(|s| s.to_str().expect("Filename isn't valid unicode!").to_owned());

        // Change ext to ".ppm" and write the image as a ppm
        let success = path.set_extension("ppm");
        assert!(success, "Failed to change extension of file name: {:?}", path);
        self.write_ppm(&path)?;

        // If the file originally had an extension, use imagemagick to convert it
        // then remove the ppm file
        if let Some(extension) = extension {
            let ppm = path.as_path().to_owned();
            path.set_extension(extension);

            // convert the file
            if let Ok(mut proc) = Command::new("convert").arg(&ppm).arg(path).spawn() {
                proc.wait().unwrap();
            } else {
                eprintln!(
                    "Error running the `convert` command! \
                     Is Image Magick installed?"
                );
            }
            // remove the ppm file
            if let Err(error) = fs::remove_file(&ppm) {
                eprint!("Error removing \"{:?}\": {}", ppm, error);
            }
        }

        Ok(())
    }

    pub fn display(&self) {
        if let Ok(mut proc) = Command::new("display").stdin(Stdio::piped()).spawn() {
            #[rustfmt::skip]
            proc.stdin
                .as_mut()
                .unwrap()
                .write_all(self.to_string().as_bytes())
                .unwrap();
            proc.wait().unwrap();
        } else {
            eprintln!("Error running `display` command! Saving file instead.");
            let name = "pic.png";
            self.write(&[name]).unwrap();
            eprintln!("Saved to `{}/{}`", PICTURE_DIR, name);
        }
    }

    pub fn clear(&mut self) {
        self.fill(self.screen_color);
    }

    pub fn fill(&mut self, c: Color) {
        for row in self.rows_mut() {
            for (color, z) in row {
                color.color(c);
                *z = f64::NEG_INFINITY;
            }
        }
    }

    pub fn plot(&mut self, px: i32, py: i32, z: f64, c: Color) {
        // Can't plot points outside the screen
        if px < 0 || px >= (self.xres as i32) || py < 0 || py >= (self.yres as i32) {
            return;
        }
        // Cast the coordinates to `usize` and
        // make (0, 0) the bottom left corner instead of the top left corner
        let (px, py) = (px as usize, self.yres - 1 - (py as usize));
        // Get the pixel and change its color and zbuffer values
        let (color, zbuffer) = &mut self[py][px];
        if z > *zbuffer {
            color.color(c);
            *zbuffer = z;
        }
    }

    // Bresenham's line algorithm
    #[allow(clippy::many_single_char_names)]
    pub fn draw_line(
        &mut self,
        (x0, y0, z0): (i32, i32, f64),
        (x1, y1, z1): (i32, i32, f64),
        c: Color,
    ) {
        // swap points if going right -> left
        if x0 > x1 {
            self.draw_line((x1, y1, z1), (x0, y0, z0), c);
            return;
        }

        let (mut x, mut y, mut z) = (x0, y0, z0);
        let a = 2 * (y1 - y0);
        let b = -2 * (x1 - x0);
        let (mut wide, mut tall) = (false, false);

        let mut d;
        let (mut loop_start, loop_end);
        let (dy_east, dy_northeast, dx_east, dx_northeast, d_east, d_northeast);

        // octants 1/8
        if (x1 - x0).abs() >= (y1 - y0).abs() {
            wide = true;
            loop_start = x;
            loop_end = x1;
            dx_east = 1;
            dx_northeast = 1;
            dy_east = 0;
            d_east = a;
            // octant 1
            if a > 0 {
                d = a + b / 2;
                dy_northeast = 1;
                d_northeast = a + b;
            }
            // octant 8
            else {
                d = a - b / 2;
                dy_northeast = -1;
                d_northeast = a - b;
            }
        }
        // octants 2/7
        else {
            tall = true;
            dx_east = 0;
            dx_northeast = 1;
            // octant 2
            if a > 0 {
                d = a / 2 + b;
                dy_east = 1;
                dy_northeast = 1;
                d_northeast = a + b;
                d_east = b;
                loop_start = y;
                loop_end = y1;
            }
            // octant 7
            else {
                d = a / 2 - b;
                dy_east = -1;
                dy_northeast = -1;
                d_northeast = a - b;
                d_east = -b;
                loop_start = y1;
                loop_end = y;
            }
        }
        let dz = (z1 - z0) / f64::from(loop_start - loop_end).abs();
        // draw points
        while loop_start < loop_end {
            self.plot(x, y, z, c);
            if (wide && ((a > 0 && d > 0) || (a < 0 && d < 0)))
                || (tall && ((a > 0 && d < 0) || (a < 0 && d > 0)))
            {
                y += dy_northeast;
                d += d_northeast;
                x += dx_northeast;
                z += dz;
            } else {
                x += dx_east;
                y += dy_east;
                d += d_east;
                z += dz;
            }
            loop_start += 1;
        }
        self.plot(x1, y1, z1, c);
    }

    pub fn draw_lines(&mut self, edges: &Matrix, c: Color) {
        // Iterate over the edge list 2 points at a time
        for edge in edges.m.chunks_exact(2) {
            self.draw_line(
                (edge[0][0] as i32, edge[0][1] as i32, edge[0][2]),
                (edge[1][0] as i32, edge[1][1] as i32, edge[1][2]),
                c,
            );
        }
    }

    pub fn draw_polygons(
        &mut self,
        polygons: &Matrix,
        reflect: Option<&Reflection>,
        lights: &Option<&[Light]>,
    ) {
        // If `reflect` and `lights` aren't provided, use the defaults
        //let reflect = reflect.unwrap_or(&self.reflection);
        //let default = self.lights.as_slice();
        //let lights = lights.as_ref().unwrap_or(&default);

        // Iterate over the edge list 3 points at a time
        for edge in polygons.m.chunks_exact(3) {
            // Get normal vector for backface culling
            let normal = Vector::calculate_normal(edge);

            if normal.z > 0.0 {
                let c = Shine::get_shine(&self, &normal, reflect, lights);
                self.scanline_convert(edge, c);
            }
        }
    }

    fn scanline_convert(&mut self, triangle: &[[f64; COLS]], c: Color) {
        assert_eq!(3, triangle.len(), "Triangles must have 3 points!");
        // order the 3 points from lowest to highest y value
        let (mut bot, mid, mut top);
        if triangle[0][1] < triangle[1][1] {
            bot = triangle[0];
            top = triangle[1];
        } else {
            bot = triangle[1];
            top = triangle[0];
        }
        if triangle[2][1] > top[1] {
            mid = top;
            top = triangle[2]
        } else if triangle[2][1] < bot[1] {
            mid = bot;
            bot = triangle[2]
        } else {
            mid = triangle[2]
        }

        // make vars immutable
        let (bot, top) = (bot, top);

        // Given a ▲BMT where b.y <= m.y <= t.y:
        // for y in b.y..t.y, draw a line from (x0, y) to (x1, y), where
        // x0 := the point along line BT with a y value of `y`
        // x1 := the point along either line BM or MT with a y value of `y`
        let (mut x0, mut x1, mut z0, mut z1) = (bot[0], bot[0], bot[2], bot[2]);
        let delta_x0 = (top[0] - bot[0]) / (top[1] - bot[1]);
        let delta_z0 = (top[2] - bot[2]) / (top[1] - bot[1]);
        // Case where bottom and middle don't have the same y value
        // check that bot[1] != mid[1]
        if (bot[1] - mid[1]).abs() > f64::EPSILON {
            let delta_x1 = (mid[0] - bot[0]) / (mid[1] - bot[1]);
            let delta_z1 = (mid[2] - bot[2]) / (mid[1] - bot[1]);
            for y in (bot[1] as i32)..(mid[1] as i32) {
                self.draw_line((x0 as i32, y, z0), (x1 as i32, y, z1), c);
                x0 += delta_x0;
                x1 += delta_x1;
                z0 += delta_z0;
                z1 += delta_z1;
            }
        }
        x1 = mid[0];
        z1 = mid[2];
        // Case where middle and top don't have the same y value
        // check that mid[1] != top[1]
        if (mid[1] - top[1]).abs() > f64::EPSILON {
            let delta_x1 = (top[0] - mid[0]) / (top[1] - mid[1]);
            let delta_z1 = (top[2] - mid[2]) / (top[1] - mid[1]);
            for y in (mid[1] as i32)..(top[1] as i32) {
                self.draw_line((x0 as i32, y, z0), (x1 as i32, y, z1), c);
                x0 += delta_x0;
                x1 += delta_x1;
                z0 += delta_z0;
                z1 += delta_z1;
            }
        }
    }
}

impl fmt::Display for Screen {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let header = mem::size_of_val("P3 {} {} 255\n{}");
        let row = self.xres * mem::size_of_val(&(Color::new(255, 255, 255).to_string() + " ")) + 1;
        let size = header + self.yres * row;
        let mut contents = String::with_capacity(size);
        for row in self.rows() {
            for (color, _) in row {
                contents.push_str(&color.to_string());
            }
            contents.push_str("\n");
        }
        write!(f, "P3 {} {} 255\n{}", self.xres, self.yres, contents)
    }
}

impl Default for Screen {
    fn default() -> Screen {
        ScreenBuilder::default().create()
    }
}

impl Index<usize> for Screen {
    type Output = [Pixel];

    fn index(&self, index: usize) -> &[Pixel] {
        let rows: Box<_> = self.rows().collect();
        rows[index]
    }
}

impl IndexMut<usize> for Screen {
    fn index_mut(&mut self, index: usize) -> &mut [Pixel] {
        let rows: Box<_> = self.rows_mut().collect();
        rows[index]
    }
}

pub struct ScreenBuilder {
    // Screen properties
    pub xres: usize,
    pub yres: usize,
    pub screen_color: Color,
    pub line_color: Color,
    pub steps_2d: usize,
    pub steps_3d: usize,
    pub specular_exp: i32,
    pub view_vector: Vector,
    pub ambient_light: Color,
    // Default values
    pub lights: Vec<Light>,
    pub reflection: Reflection,
}

impl ScreenBuilder {
    pub fn create(mut self) -> Screen {
        // normalize the vectors
        self.view_vector.normalize();
        for light in &mut self.lights {
            light.pos.normalize();
        }
        Screen {
            pixels: vec![(self.screen_color, f64::NEG_INFINITY); self.xres * self.yres],
            xres: self.xres,
            yres: self.yres,
            screen_color: self.screen_color,
            line_color: self.line_color,
            steps_2d: self.steps_2d,
            steps_3d: self.steps_3d,
            specular_exp: self.specular_exp,
            ambient_light: self.ambient_light,
            lights: self.lights,
            view_vector: self.view_vector,
            reflection: self.reflection
        }
    }
}

impl Default for ScreenBuilder {
    fn default() -> ScreenBuilder {
        ScreenBuilder {
            xres: 500,
            yres: 500,
            screen_color: consts::BLACK,
            line_color: consts::GREEN,
            steps_2d: 100,
            steps_3d: 100,
            specular_exp: 4,
            ambient_light: Color::new(50, 50, 50),
            lights: vec![Light::new(Vector::new(0.5, 0.75, 1.), consts::WHITE)],
            view_vector: Vector::new(0., 0., 1.),
            reflection: Reflection::new(
                Shine::new(0.1, 0.1, 0.1), // Ambient
                Shine::new(0.5, 0.5, 0.5), // Diffuse
                Shine::new(0.5, 0.5, 0.5), // Specular
            )
        }
    }
}
