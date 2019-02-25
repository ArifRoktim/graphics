use std::error::Error;

mod screen;
mod matrix;

use screen::Screen;
use screen::Color;
use matrix::Matrix;

const COLUMNS: usize = 500;
const ROWS: usize = 500;

fn main() -> Result<(), Box<dyn Error>> {
    let m0 = Matrix::new(1);
    println!("\nTesting new(). m0=\n{}", m0);
    let mut m2 = Matrix::new(0);
    m2.add_point(1., 2., 3.);
    m2.add_point(4., 5., 6.);
    println!("\nTesting add_point(). m2=\n{}", m2);
    let mut m1 = Matrix::new(4);
    m1.ident();
    println!("\nTesting ident(). m1=\n{}", m1);
    m1.mult(&mut m2);
    println!("\nTesting mult(). m1 * m2=\n{}", m2);
    let mut m1 = Matrix::new(0);
    //m1.add_point(1., 2., 3.);
    //m1.add_point(4., 5., 6.);
    //m1.add_point(7., 8., 9.);
    //m1.add_point(10., 11., 12.);
    m1.add_edge(1., 2., 3., 4., 5., 6.);
    m1.add_edge(7., 8., 9., 10., 11., 12.);
    println!("\nTesting add_edge(). m1=\n{}", m1);
    m1.mult(&mut m2);
    println!("\nTesting mult(). m1 * m2 =\n{}", m2);
    
    // Make an image
    let mut screen = Screen::new(Color::new(234, 100, 89));

    //let mut m = Matrix::new(0);
    //m.add_edge(0., 0., 0., 255., 255., 255.);
    //screen.draw_lines(m, Color::black())?;

    //let mut m = Matrix::new(0);
    //add_semi_circle(&mut m, 250, 250, 100, true);
    //fill_circle(&mut m, 100, 100, 50);
    //screen.draw_lines(m, Color::black())?;

    let mut m = Matrix::new(0);
    add_semi_circle(&mut m, 250, 250, 100, true);
    screen.draw_lines(m, Color::black())?;

    let mut eyes = Matrix::new(0);
    fill_circle(&mut eyes, 320, 250, 27);
    screen.draw_lines(eyes, Color::new(184, 247, 250))?;

    let mut eyes = Matrix::new(0);
    fill_circle(&mut eyes, 260, 250, 27);
    screen.draw_lines(eyes, Color::new(184, 247, 250))?;

    let mut fingers = Matrix::new(0);
    add_semi_circle(&mut fingers, 340, 150, 10, false);
    screen.draw_lines(fingers, Color::black())?;

    let mut fingers = Matrix::new(0);
    add_semi_circle(&mut fingers, 320, 150, 10, false);
    screen.draw_lines(fingers, Color::black())?;

    let mut fingers = Matrix::new(0);
    add_semi_circle(&mut fingers, 300, 150, 10, false);
    screen.draw_lines(fingers, Color::black())?;

    let mut fingers = Matrix::new(0);
    add_semi_circle(&mut fingers, 280, 150, 10, false);
    screen.draw_lines(fingers, Color::black())?;

    // ====
    let mut fingers = Matrix::new(0);
    add_semi_circle(&mut fingers, 340, 151, 10, false);
    screen.draw_lines(fingers, Color::black())?;

    let mut fingers = Matrix::new(0);
    add_semi_circle(&mut fingers, 320, 151, 10, false);
    screen.draw_lines(fingers, Color::black())?;

    let mut fingers = Matrix::new(0);
    add_semi_circle(&mut fingers, 300, 151, 10, false);
    screen.draw_lines(fingers, Color::black())?;

    let mut fingers = Matrix::new(0);
    add_semi_circle(&mut fingers, 280, 151, 10, false);
    screen.draw_lines(fingers, Color::black())?;

    let mut m = Matrix::new(0);
    m.add_edge(150., 250., 0., 150., 0., 0.);
    m.add_edge(350., 223., 0., 350., 150., 0.);
    m.add_edge(330., 200., 0., 330., 150., 0.);
    m.add_edge(310., 200., 0., 310., 150., 0.);
    m.add_edge(290., 200., 0., 290., 150., 0.);
    m.add_edge(270., 200., 0., 270., 150., 0.);
    m.add_edge(320., 140., 0., 320., 0., 0.);
    screen.draw_lines(m, Color::black())?;


    screen.write("out.ppm")?;
    Ok(())
}

fn add_semi_circle(m: &mut Matrix, h: usize, k: usize, r: usize, top: bool) {
    let start: usize;
    let end: usize;
    if top {
        start = k;
        end = k + r;
    } else {
        start = k - r;
        end = k;
    }
    for x in (h-r)..(h+r) { for y in start..end{
        let sqrt: f64 = (r as f64).powf(2.) - (x as f64 - h as f64).powf(2.);
        let val: f64;
        if top {
            val = k as f64 + sqrt.sqrt();
        } else {
            val = k as f64 - sqrt.sqrt();
        }
        if y == val as usize {
            m.add_point(x as f64, val, 0.);
        }
    }}
}

fn add_circle(m: &mut Matrix, h: usize, k: usize, r: usize) {
    add_semi_circle(m, h, k, r, true);
    add_semi_circle(m, h, k, r, false);
}

fn fill_circle(m: &mut Matrix, h: usize, k: usize, r: usize) {
    for x in (h-r)..(h+r) { for y in (k-r)..(k+r){
        let sqrt: f64 = (r as f64).powf(2.) - (x as f64 - h as f64).powf(2.);
        let pos = k as f64 + sqrt.sqrt();
        let neg = k as f64 - sqrt.sqrt();
        if y == pos as usize {
            m.add_point(x as f64, pos, 0.);
        }
        if y == neg as usize {
            m.add_point(x as f64, neg, 0.);
        }
    }}
}
