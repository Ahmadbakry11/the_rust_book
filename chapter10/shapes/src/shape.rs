pub mod circle;
pub mod rectangle;
pub mod triangle;
pub mod line_pair;

pub trait Shape {
    fn area(&self) -> f64;

    fn circumference(&self) -> f64;
}
