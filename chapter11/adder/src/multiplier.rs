fn multiply(x: i32, y: i32) -> i32 {
    x * y
}

#[cfg(test)]

mod tests {
    use super::*;

    #[test]

    fn it_works() -> Result<(), String> {
        let result = multiply(9, 9);

        if result == 81 {
            Ok(())
        } else {
            Err(String::from("The multiplication of 9 and 9 should be 81"))
        }
    }
}