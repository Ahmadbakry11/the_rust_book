#[derive(Debug)]
enum IpAddrKind {
    V4,
    V6,
}

enum IpAddress {
    V4(String),
    V6(String),
}

enum IpAddrType {
    V4(u8, u8, u8, u8),
    V6(String),
}

struct Position {
    x: i32,
    y: i32,
}

// The below Message enum has 4 variants and each variant can have any type of data
// like structs, String, i32 etc.
enum Message {
    Quit,
    Move(Position),
    Write(String),
    ChangeColor(i32, i32, i32),
}

impl Message {
    fn call(&self) -> (Position, String) {
        (Position {x: 12, y: 19}, String::from("called enum"))
    }
}

struct IpAddr {
    kind: IpAddrKind,
    address: String
}

// a nice example to indicate how to define methods for enums
struct  Rectangle {
    width: f64,
    height: f64
}

struct Circle {
    radius: f64
}

impl Rectangle {
    fn area(&self) -> f64 {
        self.width * self.height
    }
}

impl Circle {
    fn area(&self) -> f64 {
        3.14 * self.radius * self.radius
    }
}
enum Shape {
    Polygon(Rectangle),
    Circular(Circle),
}

impl Shape {
    fn area(&self) -> f64 {
        match self {
            Shape::Polygon(Rectangle { width, height}) => width * height,
            Shape::Circular(Circle { radius}) => 3.14 * radius * radius,
        }
    }
}

fn main() {
    // here is how we can create variables using the enums.
    // let four = IpAddrKind::V4;
    // let six = IpAddrKind::V6;

    // dbg!(four);
    // dbg!(six);

    // we can assign data using structs like below:
    let ip_address = IpAddr {
        kind: IpAddrKind::V4,
        address: String::from("127.0.0.1")
    };

    println!("Our network has the address: {}", ip_address.address);

    // we can attach data to each variant in the enum

    let home = IpAddress::V4(String::from("127.0.0.1"));

    // each variant of a struct can have different types and amounts of associated data.
    // we can represent v4 as a four numeric values between 0 & 255 and v6 only as a String which is not 
    // possible using structs

    let home_network = IpAddrType::V4(127, 0, 0,1);
    let work_network: IpAddrType = IpAddrType::V6(String::from("::1"));

    let rectangle = Rectangle {
        width: 10.25,
        height: 25.25
    };

    let circle = Circle {
        radius: 25.29
    };

    let shape = Shape::Polygon(rectangle);
    let shape2 = Shape::Circular(circle);
    println!("The area of the Rectangle is {}", shape.area());
    println!("The area of the Circle is {}", shape2.area());

    // ===========================================================
    // The Option Enum and Its Advantages Over Null Values
    /*
        Let's talk about the Null pointer exception and the Billion dollars mistake.

        In other programming languages like java for example, they allow to
        add a reference or a pointer to data that do not exist in memory.
        for example see the below java code:

        String name = null;
        System.Out.println(name.length());

        The above code will raise Null pointer exception.This comes from the fact that we created a pointer
        to a value the does not exist in memory.This will lead to many bugs and already that happened.

        In rust, it protects you from having such errors, because Rust does not have a null value.
        At compile time, the compiler checks against each variable has a specific value that can not be unexpected.
        This happens by using the Option enum
        Option enum has 2 variants, Some(T) and None.And the Option Enum itself is by default coming with the 
        Standard library in the prelude and no need to import that type and so are the two variants
        Some(T) and None.You can use them anytime without the Option:: prefix.
        The Option enum is defined in the standard library in the form
        
        enum Option<T> {
            None,
            Some(T)
        }

        Rust uses Option<T> to represent values that may or may not exist.
        This enforces safety at compile time.

        Option<T> is a type that can be either Some(T) (a valid value) or None(no value).

        Rust forces us to handle both cases Some and None and consequently, we can not accidently dereference a Null Pointer.
        Because there is no pointer pointing to Nothing.
    
    */ 
    
    //  we have not explicitly annotated some_number, but rust infers that it is i32 because of 5
    let some_number = Some(5);

    let some_char = Some('a');
    // but in case of None, the value is not provided and the compiler can not infer the type,
    // so, we need to annotate the type.
    let non_value: Option<i32> = None;
    
    // Another example for accessing values in the Some variant.

    let x:i8 = 5;   //x is an integer
    let some_value: Option<i8> = Some(5);  //some_value is an optional integer; So, it can be either an integer or it can be nothing.

    // we can not add these values together because they are from different types.
    // one is from Option type and the other is an integer.
    // to extract the value from the variant some_value.

    // the below code will not compile.

    // let sum = x + some_value;

    // to make the above compile, we need to extract value from the variant
    // we need to handle bothe cases where the some_value variant has value or does not have value(None)
    // Option enum has methods to handle such case
    let sum = x + some_value.unwrap_or(0); //if some_value has a value use it otherwise use a default value which is 0 in this case.
    
    println!("The sum of the integer and optional value is {sum}");

}
