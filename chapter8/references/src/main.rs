fn main() {
    // let mut v1: Vec<&str> = Vec::new();

    // v1.push("value");

    // println!("{:?}", v1);

    // let mut v2: Vec<u32> = vec![1,2,3];

    // let &mut third = &v2[2];   //ref to immutable data
    
    // third += 2;

    // println!("{third}");

    // println!("Result of sum is: {}", *third + 2);

    // let x = third;

    // println!("X value {x} is a copy of third value {third} because both are saved on the stack");

    let mut s = String::from("Hello");

    println!("The length of the string is {}", calculate_length(&mut s));  //borrowing has not been invalid
    
    change_str(&mut s); //borrowing has not been invalid because ownership of the ref is temporairly available
    // and is valid until the call of the function change_str ends

    println!("{s}");

    println!("The string now is {s}"); //borrowing has not been invalid

    let r1 = &s;

    println!("The string length now is {}", r1.len());

    let r2 = &s;

    println!("The string length now is {}", r2.len());

    println!("{r1}");
    println!("{r2}");

    let r3 = &s;
    let r4 = &s;

    println!("The refs are {r3}, {r4}");

    println!("{r1}");
    println!("{r2}");

    // the scope of r1 & r2 ends here

    let r5 = &mut s;

    println!("The ref is {r5}");


    trim_str(s);

}

fn calculate_length(s: &mut String) -> usize {
    s.len()
}

fn change_str(some_string: &mut String) {
    some_string.push_str(" World!");
}

fn trim_str(str: String) {
    println!("The trimmed text is {}", str.trim());
}