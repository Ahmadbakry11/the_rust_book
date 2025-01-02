const PI: f64 = 3.14;

struct Circle {
  radius: f64,
}

impl Shape for Circle {
  fn area(&self) -> f64 {
      PI * powi(self.radius, 2);
  }
}