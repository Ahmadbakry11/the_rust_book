mod front_of_house;
mod back_of_house;
// mod back_of_house;
use crate::back_of_house::customer;


fn deliver_order() {
    println!("I am delivering the order");
}


fn main() {

   customer::fix_incorrect_order();  //relative path

   crate::back_of_house::customer::fix_incorrect_order(); //absolute path

   let breakfast = crate::back_of_house::Breakfast::summer("Rye");

   println!("My meal was a breakfast and it contained toast of type {} and fruits of", breakfast.toast);

   println!("The breakfast was {breakfast:?}");
}
