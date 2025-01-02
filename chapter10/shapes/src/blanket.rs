
/*
    An example of using blanket implementation
    implement a trait for a struct of a specific Type
    that implement another trait

    I am implememnting a trait Numeric
    only for Value{} struct of type T
    That implements num_traits

*/ 
use std::ops::Mul;
use num_traits::{PrimInt, Zero, One};

pub trait Numeric {
    fn dublicate(&self) -> Self;
}

pub struct Value<T> {
  pub x: T
}

impl<T> Value<T> {
    pub fn new(x: T) -> Self {
      Self { x }
    }
}

impl<T: PrimInt  + Zero + One + Mul> Numeric for Value<T> {
    fn dublicate(&self) -> Self {
      Self {

        // you need to convert 2 to Type T
        // inorder to complete the multiplication process
        // They have to be from the same type
        // T::from(2) ===> convert 2 to type  T
        // This is a Result, WHICH WILL Panic if the conversion fails
        x: self.x  * T::from(2).unwrap()
      }
    }
}
