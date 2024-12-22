pub fn fix_incorrect_order() {
  println!("I am fixing the order");
  cook_order();
  crate::deliver_order();
}

fn cook_order() {
  println!("I am cooking the order!");
}