/* 
    We can control what can be done or what happens when a value goes
    out of scope.Just implement the Drop trait for the type
    and defin a function called drop that takes a mutable ref of self.
    That drop trait is preluded with the std library.
*/

struct Customer {
    data: String,
}

impl Drop for Customer {
    fn drop(&mut self) {
        println!("The customer has been dropped with data:  {}", &self.data);
    }
}

fn main() {
    let customer1 = Customer {
        data: String::from("I am the customer 1"),
    };

    {
        let customer2 = Customer {
            data: String::from("I am the customer no.2")
        };
    }

    /* 
        You might want to force drop the value early instead of waiting for it to
        be out of the scope.Like having locks and you need to release that lock to acquire using
        the value.

        we can not manually call the drop method.

        instead we call std::mem::drop

        just pass the drop function the value you need to drop.
        
    */

    let customer3 = Customer {
        data: String::from("I am the customer 3"),
    };

    println!("Customer 3 created");
    drop(customer3);
    println!("Customer 3 has been drpped before the ned of main()");

}

// customer2 goes out of scope first before customer1
