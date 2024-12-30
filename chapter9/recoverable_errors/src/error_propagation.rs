use std::io::{self, Read};
use std::fs::{self, File};

pub fn read_username_from_a_file(file_path: &String) -> Result<String, io::Error> {
    let username_file_result = File::open(file_path);

    let mut username_file = match username_file_result {
        Ok(file) => file,
        Err(e) => return Err(e), 
    };

    let mut username = String::new();

    match username_file.read_to_string(&mut username) {
      Ok(_) => Ok(username),
      Err(e) => Err(e),
    }
}
/*
    let mut username_file = File::open(file_path)?;
    using the ? operator will do the below:
    The result of the expression File::open(file_path) is of type Result.
    If the expression above evaluates to Ok(file).The value inside Ok() will be returned.
    Otherwise, it will return Err(error) as a Result.Since the return value of the function is a Result.
*/ 
pub fn shorthanded_read_username_from_a_file(file_path: &String) -> Result<String, io::Error> {
    let mut username_file = File::open(file_path)?;
    let mut username = String::new();

    username_file.read_to_string(&mut username)?;
    Ok(username)
}

// we can make the above function simpler by chaining method calls like below.

pub fn chained_read_username_from_a_file(file_path: &String) -> Result<String, io::Error> {
    let mut username = String::new();
    File::open(file_path)?.read_to_string(&mut username)?;

    Ok(username)
}

/*
  Reading a file and assign the reading result to a string is a common operation in Rust
  and hemce it is included in the standard library like below
*/ 

pub fn standard_read_username_from_a_file(file_path: &String) -> Result<String, io::Error> {
    fs::read_to_string(file_path)
}

/*
  We can use the ? with any function that has a return type of 
  Result<T, E>
  Option<T> ====> 
    In that case if the Option<T> evaluates to None, an early return with None will happen
    Otherwise, the return value will be the value inside Some(T)
*/ 

pub fn last_char_of_the_first_line(text: &str) -> Option<char> {
    text.lines().next()?.chars().last()
}