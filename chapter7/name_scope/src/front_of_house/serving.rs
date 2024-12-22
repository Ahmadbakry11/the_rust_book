use crate::front_of_house;

fn take_order() {
  println!("I am taking order");
}

pub fn serve_order() {
  println!("I am serving order");
  // modules hosting and serving both are siblings and no need to make any of
  // them as public to access the other.
  // but to use any function defined in any of them, it needs to be public

  // to access the hosting module use super or front_of_house as it is the parent module.s
  super::hosting::add_to_waitlist();
  front_of_house::hosting::add_to_waitlist();
}

fn take_payment() {
  println!("I am taking payments");
}