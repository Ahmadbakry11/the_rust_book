pub fn add(left: u64, right: u64) -> u64 {
    left + right
}

pub fn prints_and_returns_10(a: i32) -> i32 {
    println!("The value passed was {a}");
    10
}

pub fn collection_sum(v: Vec<i32>) -> i32 {
    let mut sum = 0;

    for i in v {
        sum += i;
    }

    sum
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
    fn one_hundred() {
        let a = add(90, 10);
        println!("it retruns 100");

        assert_eq!(a, 100);
    }

    #[test]
    fn one_hundred_and_fourty() {
        let a = add(100, 40);
        assert_eq!(a, 140);
    }
    
    #[test]
    fn returns_10() {
        let a = prints_and_returns_10(9);
        assert_eq!(a, 10);
    }
    
    #[test]
    fn doesnot_return_10() {
        let a = prints_and_returns_10(9);
        assert_ne!(a, 10);
    }

    #[test]
    #[ignore]

    fn expensive_test() {
        let result = collection_sum(vec![1,2,3,4,5]);
        
        assert_eq!(result, 15);

    }
}
