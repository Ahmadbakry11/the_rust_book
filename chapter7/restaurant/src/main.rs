mod front_of_house {
    pub mod hosting {
        pub fn add_to_waitlist() {
            println!("Added to waitlist");
        }

        fn seat_at_table() {
            // sibling functions within the same modules are accessible to each other.
            add_to_waitlist();
            println!("Customer is @ seat 2");
        }
    }

    mod serving {
        fn take_order() {
            println!("I am taking order");
        }

        fn serve_order() {
            println!("I am serving order");
        }

        fn take_payment() {
            println!("I am taking payments");
        }
    }
}

mod dinning {
    use crate::front_of_house::hosting;
    fn eat_at_restaurant() {

        // call to add_to_waitlist
        // we need to make a call to add_to_waitlist
        // it is in the same level where the parent module is defined
        // so we can add the relative path like below:
        // in order to call a child  module, we need to make it oublic, because it is private by default.
        // child items are private to parent while parent items are public (accessible) to child 
        // based on that we need to make the function add_to_waitlist public too.
    
        hosting::add_to_waitlist();  //
        crate::front_of_house::hosting::add_to_waitlist();  //absolute path
    }
}

fn deliver_order() {
    println!("I am delivering the order");
}

mod back_of_house {
    #[derive(Debug)]
    pub struct Breakfast {
        pub toast: String,
        seasonal_fruit: String
    }

    impl Breakfast {
        // if we deined one or more of the Struct fields as private
        // we need to implement an associated function to create 
        // instances from this struct, that declare default values for those private fields

        pub fn summer(toast: &str) -> Breakfast { //summer is an associated function
            Self {
                toast: String::from(toast),
                seasonal_fruit: String::from("Peaches")
            }
        }
    }


    pub mod customer {
        pub fn fix_incorrect_order() {
            println!("I am fixing the order");
            cook_order();
            crate::deliver_order();
        }
    
        fn cook_order() {
            println!("I am cooking the order!");
        }
    }
}

mod meal {
    
    pub mod generator {
        pub struct Food{}

        pub fn genearate_meal() -> Food {
            Food{}
        }
    }
}

mod invoice {
    pub struct PaySlip {}
    pub mod generator {
        pub fn genearate_invoice(){

        }
    }
}


fn main() {

   back_of_house::customer::fix_incorrect_order();  //relative path

   crate::back_of_house::customer::fix_incorrect_order(); //absolute path

   let breakfast = crate::back_of_house::Breakfast::summer("Rye");

   println!("My meal was a breakfast and it contained toast of type {} and fruits of", breakfast.toast);

   println!("The breakfast was {breakfast:?}");
}
