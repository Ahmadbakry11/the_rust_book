use crate::garden::vegetables::VegiKinds;

pub struct Recipe {
  pub name: String,
  pub duration: u32,
  pub ingredients: [VegiKinds; 2]
}

impl Recipe {
    pub fn show_details(&self) {
      println!("Our today's dish is {}", self.name);
      println!("It will take from us {} seconds to prepare", self.duration);
      println!("It has the following ingredients: ");
    
      for ingredient in &self.ingredients {
          println!("{ingredient:?}");
      }
    }
}