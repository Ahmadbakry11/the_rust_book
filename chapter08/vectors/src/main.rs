#[derive(Debug)]
enum SpreadSheetCell {
    Int(i32),
    Text(String),
    Float(f64)
}

fn main() {
    let v1= vec![1,2,3];
    
    for i in &v1{
        println!("{i}");
    }

    // let's have a mutable vector
    // and duplicate its contents
    let mut v = vec![1,2,3,4,5];

    for i in &mut v {
        // i itsled is a ref, so applying math operations need to access the value itself
        // this is by dereferencing it
       *i *= 2;
    }

    println!("The first 3 values are {}, {}, {}", &v[0], &v[1], &v[2]);

    // getting values using get

    let first = &v[0];

    println!("{first}");

    let second: &Option<&i32> = &v.get(1);   
    //the reason of why <&i32> instead of <i32>
    // lies in that the vector gives you a ref to the requested element if it exists, and not the element itself
    // because the vector owns its element and if it gives you the elements, the ownership will be moved to you.
    // also, if it gives you the element instead of a ref to the element and since elements in this case are of type
    // i32, so copies will be made there and there will be lots of them in case of large vectors

    //we can perform matching against the Option variants like below

    match second {
        Some(x) => println!("The requested element is {x}"),
        None => println!("No element found at the given index")
    }

    // accessing an element with index out of bound using [], will make the app panic
    // by using get, we will get none

    let none = v.get(100);
    println!("{none:?}");

    // using enums with vectors
    // Vectors can store many values of the same data type.In case we need to store multiple data types,
    // We can use enums with each variant having that type.
    // In case you know of the exhaustive types (variants) of an enum, an enum is not your option
    
    let row = vec![
        SpreadSheetCell::Int(90),
        SpreadSheetCell::Text(String::from("Hi")),
        SpreadSheetCell::Float(90.87),
    ];
    
    let text_cell = &row[1];
    println!("The text in the row is: {text_cell:?}");


}