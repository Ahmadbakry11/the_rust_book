
mod front_of_house {
    pub mod hosting {
        pub fn add_to_waitlist() {
            println!("We are adding the customer to waitlist!");
        }
    } 
}

pub use self::front_of_house::hosting;

fn eat_at_restaurant() {
    hosting::add_to_waitlist();
}

