use crate::garden::vegetables::Vegi;
use crate::garden::vegetables::VegiKinds;
use crate::recipe::Recipe;
pub mod garden;
pub mod recipe;

fn main() {
    let tomato =  VegiKinds::Tomato(Vegi {
        color: String::from("Red"),
        weight: 88.9
    });

    let lime =  VegiKinds::Lemon(Vegi {
        color: String::from("Yellow"),
        weight: 22.2,
    });

    let recipe1 = Recipe {
        name: String::from("Pasta"),
        duration: 99,
        ingredients: [tomato, lime],
    };

    recipe1.show_details();
}
