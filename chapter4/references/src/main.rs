fn main() {
    // literla string
    let s = String::from("Hi");
    let y = s;  // move s to y
    println!("{y}"); // s is no longer available, it moved
    
    // you can define multiple immutable refs at the same time
    // here refs are refs to immutable data
    let z = &y; // z is an immutable ref to y: on the stack;
    let h = &y; // h is an immutable ref to y: on the stack;

    let k = z;  // k is a copy of k, it is on the stack
    println!("{z}, {k}, {h}"); // both are available here.

    // mutable strings

    let mut m = String::from("Hi");
    // let y = m;
    // println!("{m}, {y}");  //this will not compile.ownership of m was moved to y

    // solution
    // do not move ownership, just add refs to the string instead of moving ownership
    
    // another way of moving ownership
    // let v = change_str(m);
    
    // m is not available here, ownership has moved
    // println!("{v}, {m}") //this wont compile
    
    // m still available, ownership not moved!
    // assign the return value which is a string to v
    let mut v = change_str(&mut m);  //here keeping ownership and just passing a ref
    // v is still available and also m
    v = v.to_uppercase();
    
    println!("The values are {m}, {v}");






}


fn change_str(some_str: &mut String)  -> String {
  some_str.push_str(" Rusty!");  //push_str return value is a unit

  format!("{some_str}") //return a string from unit
}

// fn dublicate_str(some_str: &mut String) {
//     let cloned_str = some_str.clone();
//     some_str.push_str(&cloned_str);
// }