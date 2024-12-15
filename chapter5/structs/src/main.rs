struct User {
    name: String,
    email: String,
    active: bool,
    sign_in_count: u64,
}

struct Document(String, u64, String);

struct AlwaysEqual;

fn main() {
    let user1: User = build_user(String::from("Aly Kamal"), String::from("aly@co.com"), 66);

    print_card(user1);
    
    // this is how we instantiate another user using the Struct
    let user2 = User {
        name: String::from("Esam Kamal"),
        email: String::from("esam@co.com"),
        active: false,
        sign_in_count: 99,
    };

    // Another way to create another instance
    // here we are using user2 values for fields not explicitly defined in user3
    // these values can be used and in the same time we can use user2 as a whole 
    let user3 = User {
        name: String::from("Nagi Kamal"),
        email: String::from("nagi@co.com"),
        ..user2
    };

    // we can use user2 as a whole, because we used fields that can be copied and not moved
    // to build the user3 instance.
    // we used active and sign_in_count, both are copied since they are saved in the stack
    // not the heap.

    print_card(user2);

    // here user2 has been moved after passing it as an argument to print_card function
    // consequently, we now are unable to use it.

    // let's come to another scenario, where we can create a user instance from another isntance
    // but using values from the source that do not have the copy trait.

    let user4 = User {
        active: true,
        sign_in_count: 99,
        ..user3
    };


    // here we used values from user3 of a String type.
    // since we used those values that do not support the copy trait,
    // we can not use the whole user3 instace anymore like below which will not compile.

    // println!("The borrowed username is: {}", user3.name);
    
    // **** Using Tuple Structs Without Named Fields to Create Different Types

    let doc = Document(String::from("Certificate"), 9999, String::from("/location/docs"));

    // we can use a ref to doc here. 
    print_doc_data(&doc);

    // below ownership has not been removed, because we are accessing a data with copy trait.
    let doc_size = doc.1;

    // so we can use doc instance here as a whole.
    // ownership is moved below and we can not use doc instance as a whole after calling print_doc_details(doc) 
    print_doc_details(doc);
    
    // the below code will not compile.
   // let doc_dir = doc.2;


   // *** Unit-Like Structs Without Any Fields
   let subject = AlwaysEqual; 



}


fn build_user(name: String, email: String, sign_in_count: u64) -> User {
    User {
        name,
        email,
        sign_in_count,
        active: true,
    }
}

fn print_card(user: User) {
    println!("The user details are: name is: {}, email: {}, active: {}, times he signed in : {} times", user.name, user.email, user.active, user.sign_in_count)

}

fn print_doc_data(doc: &Document) {
    println!("The document is a {} with a size of {} and is located at {}", doc.0, doc.1, doc.2);
}

fn print_doc_details(doc: Document) {
    println!("The details are {}", doc.0)
}