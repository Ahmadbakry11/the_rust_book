use std::fmt::Display;

pub struct Point<T, U> {
  pub x: T,
  pub y: U,
}

impl<T, U> Point<T, U> {
  pub fn new(x: T, y: U) -> Self {
    Self {x, y}
  }
}

impl<T: PartialOrd + Display, U: PartialOrd + Display> Point<T, U> {
    pub fn cmp_display(&self) -> String  
      where T: PartialOrd<U> //here we are caaling this condition only in case of T, U are of the same type.
      {
        if self.x > self.y {  //you can not use the cmp without having the two of the same type.
          return String::from("High");
        } else {
          return String::from("Low");
        }
    }
}