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


pub mod customer;