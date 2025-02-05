fn main() {
    let s1 = gives_ownership();

    println!("The first string is {s1}");

    let s2 = String::from("Hello");

    let s3 = takes_and_gives_ownership(s2);

    let (s4, length) = calculate_length(s1);

    println!("The length od the of string {} is: {}", s4, length);
}


fn gives_ownership() -> String {
    let some_string = String::from("Hola");

    some_string
}

fn takes_and_gives_ownership(some_string: String) -> String {
    some_string.to_uppercase()
}

fn takes_ownership(some_string: String) {
    println!("{some_string} after havng ownership moved!");
}

fn calculate_length(some_string: String) -> (String, usize) {
    let length = some_string.len();

    (some_string, length)
}