use crate::shape::Shape;
pub struct Triangle {
    pub side1: f64,
    pub side2: f64,
    pub side3: f64,
}

impl Shape for Triangle {
    fn area(&self) -> f64 {
        self.side1 * self.side2 * self.side3
    }

    fn circumference(&self) -> f64 {
        self.side1 + self.side2 + self.side3
    }
}
