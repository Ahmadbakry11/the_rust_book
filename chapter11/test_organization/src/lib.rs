pub fn add(left: u64, right: u64) -> u64 {
    left + right
}

pub fn adds_two(a: i32) -> i32 {
    internal_adder(a, 2)
}

fn internal_adder(x: i32, y: i32) -> i32 {
    x + y
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }

    #[test]
    fn it_adds_two() {
        let a = adds_two(2);
        assert_eq!(a, 4);

    }
    
    // private functions can be tested in Rust
    #[test]
    fn it_internally_adds_numbers() {
        let a = internal_adder(2, 2);
        assert_eq!(a, 4);
    }
}


