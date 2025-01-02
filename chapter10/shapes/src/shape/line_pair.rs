use std::fmt::Display;

use super::Shape;


pub struct Pair<T> {
  x: T,
  y: T,
}

impl<T> Pair<T> {
    fn new(x: T, y: T) -> Self {
      Self {
        x,
        y
      }
    }
}

impl<T: PartialOrd + Display> Pair<T> {
  fn cmd_compare(&self) {
    if self.x > self.y {
      println!("The line x is greater than line y: {}", self.x);
    } else if  self.x < self.y {
      println!("The line y is greater than line x {}", self.y); 
    } else {
      println!("The two lines are equal"); 
    }
  }
}
