pub fn add_to_waitlist() {
  println!("Added to waitlist");
}

fn seat_at_table() {
  // sibling functions within the same modules are accessible to each other.
  add_to_waitlist();
  // we need to use a function from another module called serving.
  super::serving::serve_order();
  println!("Customer is @ seat 2");
}