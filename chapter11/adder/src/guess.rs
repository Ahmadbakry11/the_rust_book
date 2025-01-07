pub struct Guess {
    value: i32,
}

impl Guess {
    pub fn new(value: i32) -> Self {

        if value > 100 {
            panic!("value should be greather than or equal to 1");
        } else if value < 1 {
            panic!("value should be less than or equal to 100");
        }

        Self {
            value
        }
    }
}

#[cfg(test)]

mod tests {
    use super::*;

    #[test]
    #[should_panic(expected = "less than or equal to 100")]
    fn guess_value_greater_than_100() {
        Guess::new(900);
    }
}