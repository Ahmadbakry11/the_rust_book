struct Rectangle {
    width: u32,
    height: u32
}

impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }

    // we can name a method with the same name of any field
    // which the struct defines

    fn width(&self) -> bool {
        self.width > 0
    }

    fn can_hold(&self, target: &Rectangle) -> bool {
        self.width > target.width && self.height > target.height
    }
    
    // square is an associated function like String::new
    // it does not take self as the first argument.

    // for example the square func below, will be called like
    // Rectangle::sqaure

    // since it is defined to return a special version of Rectangle
    // we can make the function signature to return a Rectangle or better
    // to return Self
    // Self is an alias for Rectangle or the type appears after impl

    // fn square(size: u32) -> Rectangle {
    //     Rectangle {
    //         width: size,
    //         height: size
    //     }
    // }

    fn square(size: u32) -> Self {
        Self {
            width: size,
            height: size
        }
    }

    // better to write the square assoc
}

fn main() {
    let rect = Rectangle {
        width: 99,
        height: 100
    };

    let rect1 = Rectangle {
        width: 70,
        height: 90
    };

    let rect2 = Rectangle {
        width: 50,
        height: 80
    };

    let rect3 = Rectangle {
        width: 75,
        height: 30
    };

    println!("The area of the rect1 is {}", rect.area());

    if rect.width() {
        println!("This rectangle is wide");
    }

    println!("Can rect1 hold rect2 ? {}", rect1.can_hold(&rect2));
    println!("Can rect1 hold rect2 ? {}", rect1.can_hold(&rect3));

    let square = Rectangle::square(19);
    println!("The area of the square is {}", square.area());

}
