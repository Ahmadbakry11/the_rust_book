fn main() {
    let mut s: String = String::from("Hello Rusty!");
    
    // let word = first_word(&s);

    // s.clear();

    // println!("{word}");

    // let hello = &s[0..5];
    // let rusty = &s[6..s.len()];

    // let s1 = &s[4..];
    // let s2 = &s[..9];
    // let s3 = &s[..];

    // println!("{}: {}: {}", s1, s2, s3);

    // println!("{}: {}: {}", hello, rusty, s);

    let slice = fi_word(&s);
    // s.clear();
    println!("{slice}");
}


fn first_word(s: &String) -> usize {
    let bytes = s.as_bytes();
    for (i, &a) in bytes.iter().enumerate() {
        if a == b' ' {
            return i
        }
    }

    s.len()
}

fn fi_word(s: &String) -> &str {
    let bytes = s.as_bytes();
    for (i, &a) in bytes.iter().enumerate() {
        if a == b' ' {
            return &s[..i];
        }
    }

    &s[..]
}