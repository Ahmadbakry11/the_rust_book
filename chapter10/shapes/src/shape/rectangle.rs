use std::fmt;

use crate::shape::Shape;
pub struct Rectangle {
    pub width: f64,
    pub height: f64,
}

impl Rectangle {}

impl Shape for Rectangle {
    fn area(&self) -> f64 {
        self.width * self.height
    }

    fn circumference(&self) -> f64 {
        (self.width + self.height) * 2.0
    }
}

