use unicode_segmentation::UnicodeSegmentation;

fn main() {
    let s = "Hello Rust!";
    let mut s2 = String::from("I said ");

    // s is a string literal and by default it is included in the Rust core
    // s is a ref and it can be converted to a String type like below

    let converted_es = s.to_string();
    let message_length = converted_es.len();  
    //calling .len() above does not move ownership
    // because len() method is implemented to take an immutable ref of s2
    // the method signature is implemented like below
    // fn len(&self) -> usize 

    println!("The length of the string literal s above is {}", message_length);

    println!("The message length od s2 is {}", s2.len());
    
    // updating a string

    // 1- using push_str()

    s2.push_str(s);

    // note that s is a ref and push_str takes a ref and consequently it does not take ownership of it and
    // using s  above allows us to use it multiple time 
    // like using the println! macro.

    println!("The greeting message is {} and it is derived from {}", s2, s);

    // push method: takes a single char and append it to the String
    let mut funny = String::from("lo");
    funny.push('l');

    println!("{funny}");

    // concatenating two strings using the plus(+) operator
    let s1 = String::from("tic");
    let s2 = String::from("tac");
    let s3 = String::from("toe");

    // let s4 = s1 + &s2;
    // println!("The concatenation result is {s4}");

    // ownership of s1 will be moved due to the process above.
    // the implementation of the add method signature is like below
    // fn add(self, s: &str) -> String {}

    // so it takes the string itself instead of a ref and that is why it takes ownership
    // and the second parameter it takes a slice and if we tried to pass a ref to String like &String
    // it performs a deref coercion and converts &s2 as &s2[..] 

    /*
        we can use the plus operator to concatenate multiple strings, 
        but it will be hard to know what is going on.
        like s5 below
    */ 

    // let s5 = s1 + "-" + &s2 + "-" + &s3; // "tic-tac-toe"

    /*
        to concatenate strings in a complicated way, it is better to use the format! macro,
        it returns a result of type String and does not affect the ownership of any of the parameters passed.
        
    */ 

    let s = format!("{s1}-{s2}-{s3}");

    println!("The concatenated string is: {s}");

    println!("It is composed of {}, {}, {}", s1, s2, s3);  //ownership of s1, s2, s3 has never been moved and so, we can use them

    // indexing into strings

    let t = &s.chars().nth(0);
    println!("{t:?}");  //an &Option(<Char>)

    let z = match t {
        Some(s) => s,
        None => &'n',
    };
    
    // z is a ref to char

    println!("The first char is {}", z);
    
    let x = nth_char_of(&s, 2);

    println!("The third char is {x}");

    // when talking about strings from the rust perspective, there are 3 ways to interpret strings
    // interpreted as a sequence of bytes
    let hello = "Здравствуйте";  //we need to iterate over hello

    for b in hello.bytes() {
        println!("{b}")  //prints 24 bytes
    }


    /*
        interpreted as scalar values(chars), assuming we have the hindi word "नमस्ते" which means Hello.
        And we need to see how this word is represented in scalar values, we can iterate over chars 
        in this word

    */ 
    
    let hindi_hello = "नमस्ते";

    for c in hindi_hello.chars() {
        //prints ['न', 'म', 'स', '्', 'त', 'े'], fourth and sixth chars 
        // are diacritics and do not make sense in their own.
        println!("{c}"); 
    }
    /*  interpreted as grapheme cluster
        This is what u and I consider letters
        It is more complex and not included by default in the rust std library.we need to
        get help of a crate called unicode-segmentation
    */ 

    let hindi_graphem = hindi_hello.graphemes(true);
    println!("{hindi_graphem:?}");

    for g in hindi_hello.graphemes(true) {
        println!("{g}");  // ["न", "म", "स", "त"]
    }
    
    /*
        So, if you need to access specific parts of a string in Rust or iterate over it or even accessing a single letter
        you need to be specific about what type of representation you want from rust (bytes, scalars, grapheme clusters)
    */ 

}


fn nth_char_of(s: &String, n: usize) -> char {
    match s.chars().nth(n) {
        Some(s) => return s,
        None => return 'n',

    }
}
