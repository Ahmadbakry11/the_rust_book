fn main() {
    // deliberately panic
    // panic!("Crash and burn!");

    println!("Here we are excuting a code that will panic");

    let v = vec![1,2,3];
    v[99];
}
