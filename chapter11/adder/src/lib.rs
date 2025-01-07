pub mod guess;
pub mod rectangle;

pub mod multiplier;

pub fn add(left: u64, right: u64) -> u64 {
    left + right
}

pub fn add_two(num: i32) -> i32 {
    num + 2
}

pub fn greet(name: &str) -> String {
    // String::from("Hello")
    format!("Hello, {}", name)
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn exploration() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
    
    #[test]
    fn another() {
        assert_eq!(1, 1);
        // panic!("Make this test fail!");
    }


    #[test]

    fn it_adds_two() {
        let result = add_two(3);
        assert_eq!(result, 5);
    }
    
    #[test]

    fn greets_by_name() {
        let result = greet("Carol");
        assert!(result.contains("Carol"), "Should print name to be greeted but was {result}");
    }

}
