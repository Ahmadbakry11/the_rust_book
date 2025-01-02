use crate::shape::Shape;

const PI: f64 = 3.14;

pub struct Circle {
    pub radius: f64,
}

impl Shape for Circle {
    fn area(&self) -> f64 {
        PI * (self.radius).powi(2)
    }

    fn circumference(&self) -> f64 {
        2.0 * PI * self.radius
    }
}
