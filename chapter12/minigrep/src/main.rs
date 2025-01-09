use std::{env, fs};

fn main() {
    let args: Vec<String> = env::args().collect();
    let search_query = &args[1];
    let file_path = &args[2];

    println!("We are seraching for {search_query}");
    println!("In the file: {file_path}");
    
    let contents = fs::read_to_string(file_path)
        .expect("We should be bale to read the file");

    println!("In the text of:\n {contents}");

}
