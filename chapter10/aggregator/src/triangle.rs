struct Triangle {
    side1: f64,
    side2: f64,
    side3: f64,
}

impl Shape for Triangle {
    fn area(&self) -> f64 {
       side1 * side2 * side3
    }
}