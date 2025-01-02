pub trait Summary {
  fn state(&self) -> bool;

  fn summarize_author(&self) -> &String;

  fn summarize(&self) -> String {
    format!("Read more: {}", self.summarize_author())
  }
}

pub trait Shape {
  fn area(&self) -> f64;
}