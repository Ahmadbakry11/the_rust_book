use std::ops::{Deref, DerefMut};



struct MyBox<T>(T);
#[derive(Debug)]
struct DataBag<T> (T);

impl<T> DataBag<T> {
    fn new(data: T) -> Self {
        Self(data)
    }
}

impl<T> Deref for DataBag<T>  {
    type Target = T;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<T> DerefMut for DataBag<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl<T> MyBox<T>  {
    fn new(x: T) -> Self {
        Self(x)
    }
}

impl<T> Deref for MyBox<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.0
    } 
}

fn main() {
    let x = 9;
    let y = &x;
    let z = Box::new(x);
    
    /*
        - z is pointing to a copied value of x.
          Because boxes allow you to store data on the Heap.

        - while y is pointing to x itself
    */ 

    assert_eq!(x, 9);
    assert_eq!(*y, 9);
    assert_eq!(*z, 9);

    let my_box = MyBox::new(x);
    assert_eq!(*my_box, 9);

    let name_ref = MyBox::new(String::from("Ali"));
    hello(&name_ref);

    let bag = DataBag::new("top secret");
    println!("{bag:?}");
   

    /*
        let's breakdown what is happening here as we are simulating the performance of the smart pointers Box<T>:

        DataBag is a normal struct(Data structure) is implemented to hold data of generic type T
        and so on implemented the new function which receives an argument of type T and return the struct DataBag

        here bag lives in the stack because it holds a data of type &str or string literal

        assuming we need to pass data dynamically to the new function like below
    */

    let secret = "Hi dear";

    // we can pass secret to the new associated function implemented for DataBag

    let secret_bag = DataBag::new(secret);

    // what if we need to get the actual data stored in DataBag using ref to instances from DataBag
    assert_eq!(*secret_bag, "Hi dear");

    // we will receive a message that DataBag can not be dereferenced.
    // So, we need to implement the Deref trait for DataBag

    /*
        impl<T> Deref for DataBag<T> {
            type Target = T;

            fn deref(&self) -> &Self::Target {
                &self.0
            }
        }

        we have a note regarding the return value of the deref method.
        deref method returns a reference to the value and not the value itself.
        This is because of the ownership rules.
        If deref returns the value itself, the value will be moved out of self and that
        is something that we do not want.
    */ 

    /*  
        Deref coercion:
        ==================
        Deref coercion converts a reference to a type that implements the Deref 
        trait into a reference to another type.

        Deref coercion is a convenience Rust performs on arguments to functions and methods, 
        and works only on types that implement the Deref trait. 
        
        It happens automatically when we pass a reference to a particular type’s value as 
        an argument to a function or method that doesn’t match the parameter type in the function or 
        method definition. A sequence of calls to the deref method converts the type we provided into the type the parameter needs.

        as an example to that.The function encode below receives a string literal.
        But what happens if we passed an arg of type String.
    */ 
    let input = String::from("Hello");
    /* 
        function revert receives an arg of type &str.It receives a string slice parameter.
        But we can call it with a ref to a value of type that implements the Dref trait.
        Like String, if we pass a ref to String &String.we are passing a ref to a value of type that implements 
        the Dref trait.A sequence of calls to deref are excuted to convert a type of &String to a type of &str.
    */ 
    let reverted_input = revert(&input);
    println!("{reverted_input}");

    /*
        Lets look to another example.
        the data type DataBag<T>, we can pass secret_bag to the revert function.
        secert_bag is an instance of Type DataBag and Deref coercion allows us to call the function revert
        with a ref to DataBag<String> that implements the Deref trait.
        so we can call revert(&secret_bag)
    */ 

    let my_secret = String::from("secret");
    let my_secret_bag = DataBag::new(my_secret);

    let reverted_secret = revert(&my_secret_bag);

    println!("{reverted_secret}");

    /*
        what rust is doing is:
            * calls deref on DataBag<String> =====Returns====> &String
            * calls deref on &String =====Returns====> &str
            * passes &str to revert 
    */ 

    /*
        Rust does deref coercion when it finds types and trait implementations in three cases:
            1- From &T to &U when T: Deref<Target=U>
            2- From &mut T to &mut U when T: DerefMut<Target=U>
            3- From &mut T to &U when T: Deref<Target=U>
        We have seen case no. 1 above

        lets talk about case no 2:

        2- from &mut T to &mut U, when T: DrefMut<Target=U>
           an example

           let mut mutable_secret = String::from("hi");
           let mutable_bag = DataBag::new(mutable_secret);

           since mutable_bag is an instance of datatype that implements the DerefMut,
           a ref of it can be passed as an argument to the function edit_male_name.
           edit_male_name(&mutable_bag);

    */  

    let mutable_secret = String::from("Kamalo");
    let mut mutable_bag = DataBag::new(mutable_secret);
    let changed_name = edit_male_name(&mut mutable_bag);
    println!("{changed_name}");

    /*
        what rust is doing is:
            * calls deref_mut on DataBag<String> =====Returns====> &String
            * passes &String to edit_male_name
    */

    /*
        3-  From &mut T to &U when T: Deref<Target=U>
            This is a more subtle and less common case. Here, a mutable reference to T (&mut T) 
            is coerced into an immutable reference to U (&U). This happens when only the 
            Deref trait (not DerefMut) is implemented for T.

            The opposite is not allowed. In rust immutable ref can not be corece to mutable refs.
            As an example convert &mut String to &str
            
    */
    let mut m = MyBox(String::from("Rust")); // MyBox<String>
    let other_secret = revert(&m);
    println!("{other_secret}");

    /*
        what rust is doing is:
            * Rust finds tht DerefMut is not imlemented for MyBox
            * calls deref on MyBox<String> =====Returns====> &String
            * calls deref on &String =====Returns====> &str
            * passes &str to revert
    */
}



fn hello(name: &str) {
    println!("Hello {name}");
}

fn revert(input: &str) -> String {
    input.chars().rev().collect()
}

fn edit_male_name(name: &mut String) -> String {
    format!("Mr.{name}")
}
