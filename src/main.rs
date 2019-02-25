//use std::error::Error;

//mod screen;
//use screen::Screen;
//use screen::Color;
//use screen::Point;

mod matrix;
use matrix::Matrix;

const COLUMNS: usize = 750;
const ROWS: usize = 750;

fn main() {
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
}
