mod error_propagation;
use std::{fs::File, io::ErrorKind};

fn main() {
    /*
        Sometimes, you do not want to stop the program immediately and you 
        just need to take some action to recover from that error.

        for example, if you open a file to read or write and if that file
        does not exist, you may need to create a new one.

        you may get help of an Enum Result<T, E>, which has 2 variants
        Ok(T) and Err(E)

        enum Result<T,E> {
            Ok(T),
            Err(E),
        }

        where T is the type of Value that will returned in case of Success
        and E represents the type of Error that will be returned in case of Failure.

        let's talk about the below scenario, where we need to open a file.
    */ 

    let greeting_file_result = File::open("hello.txt");

    let greeting_file = match greeting_file_result {
        Ok(file) => file,
        Err(error) => match error.kind() {
            ErrorKind::NotFound => match File::create("hello.txt") {
                Ok(fc) => fc,
                Err(e) => panic!("Problem, creating the file {e:?}"),
            },

            other_error => panic!("Problem opening the file! {other_error:?}"),
        },
    };

    println!("The requested file is {greeting_file:?}");

    /*
        Anatomy of the above code:
        ===========================
        The generic parameter T is filled with type std::fs::File, which is a file handle
        The generic parameter E is filled with type std::io::Error

        greeting_file is a file handle that can be used to either read or write.

        The Err(error) variant returned carries an error inside, which is a struct of type
        io::Error and implements the method Kind()

        calling error.Kind() has a return type of io::ErrorKind that is provided by the standard library.

        The above code has too many match expressions and can be cleaned as following:
    */ 

    let invoice_file = File::open("invoices.txt").unwrap_or_else(|error| {
        if error.kind() == ErrorKind::NotFound {
            File::create("invoices.txt").unwrap_or_else(|e| {
                panic!("Problem creating the file: {e:?}");
            })
        } else {
            panic!("Problem openong the file: {error:?}"); 
        }
    });

    println!("The file containing invoices is: {invoice_file:?}");

    /*
        Shortcuts for Panic on Error: unwrap and expect
        The Result enum has many diffrent helper method that can replace match, one of them is the unwrap()

        calling unwrap() on a Result<T, E> will return a value of type T or call panic! in case of error
    */ 

    let bills_file = File::open("payments.txt").unwrap();
    println!("The bills file is {bills_file:?}");

    /*
        Also, there is another alternative to match which is expect.
        expect() method is like unwrap expect it receives an error message explaining why 
        the code failed.
    */

    let report = File::open("reports.txt")
        .expect("A file named report.txt should be existing in this path");

    println!("The reports file is {report:?}");
    
    let file_path = String::from("sms.txt");

    let result = error_propagation::read_username_from_a_file(&file_path);
    println!("The function call result is {result:?}");

    // Error propagation using the ? operator.
    let shorthanded_call_result = error_propagation::shorthanded_read_username_from_a_file(&file_path);

    println!("The shorthanded function call result is {shorthanded_call_result:?}");

    let chained_call_result = error_propagation::chained_read_username_from_a_file(&file_path);

    println!("The chained function call result is {chained_call_result:?}");

    let standard_call_result = error_propagation::standard_read_username_from_a_file(&file_path);

    println!("The standard function call result is {standard_call_result:?}");

    let text = "Hello Guys.\nThis is the second line.\nAnd this is the third";
    
    println!("=================================Examining the lines method=======================================");
    let text_result = text.lines();  //returns an iterator over the text slice.

    println!("{text_result:?}");

    // let last_char = text.chars().last();  //.chars() returns an iterator .last() returns an Option<char>

    let last_char = error_propagation::last_char_of_the_first_line(text);

    println!("The last char of text is {last_char:?}");

    /*
        Assuming we have the function main which has that line of code which uses the ? operator

        fn main() {
            let greeting_file = File::open("hello.txt")?;
        }

        The above code will not compile as the ? operator works only with functions that return 
        Option<T>, Result<> and so on.
        But the main function returns () unit, so it will not compile
        But luckily the main func can return also a type of Result<(), E>
        
        // the below code will compile

        fn main() -> Result<(), Box<dyn Error>> {
            let greeting_file = File::open("hello.txt")?;
            Ok(())
        }

        Box<dyn Error> means it can return return any type of error.

        when main returns with a type of Result<(), E>, the executable will return 0 for Ok(())
        and a nonezero value for a type of E. Just like C executables.

        C executables return 0 for success and programs error return integer values other than 0.


    */


}
