pub mod shape;
pub mod point;
pub mod blanket;

use std::fmt::{Debug, Display};


use blanket::Numeric;
pub use point::Point;
pub use shape::circle::Circle;
pub use shape::rectangle::Rectangle;
pub use shape::triangle::Triangle;
pub use blanket::Value;

use shape::Shape;

fn main() {
    let rectangle = Rectangle {
        width: 9.9,
        height: 8.2,
    };

    let triangle = Triangle {
        side1: 7.2,
        side2: 5.2,
        side3: 6.3,
    };

    let circle = Circle { radius: 12.5 };

    println!("The area of the triangle is {}", triangle.area());
    println!("The area of the rectangle is {}", rectangle.area());
    println!("The area of the circle is {}", circle.area());

    info_card(&triangle);
    info_card(&circle);

    shape_info(&circle);

    let point = Point::new(19, 7);

    let cmp_result = point.cmp_display();

    println!("The comparison result is {}", &cmp_result);

    let value = Value::new(9);
    let new_value = value.dublicate();
    println!("This is the duplicated value {}", new_value.x);

    // print_details(&rectangle);

}

fn info_card(shape: &impl Shape) {
    println!("The shape details are, area is: {} and circumference is {}", shape.area(), shape.circumference());
}

// trait bound syntax

fn shape_info<T: Shape>(shape: &T) {
    println!("The shape info is, area: {}, circumference: {}", shape.area(), shape.circumference());
}

fn mixed_shapes(item1: &impl Shape, item2: &impl Shape) {
    println!("item1 info is {}", item1.area());
    println!("item2 info is {}", item2.area());
}

fn mix_shape_areas<T: Shape, U: Shape>(t: &T, u: &U) {
    println!("item1 info is {}", t.area());
    println!("item2 info is {}", u.area());
}

fn print_details(item: &(impl Shape + Display)) {
    println!("item info is {}", item.area());
    println!("{item}");
}

fn show_details<T: Shape + Display>(item: &T) {
    println!("item info is {}", item.area());
    println!("{item}");
}

// multiple traits

fn list_info<T: Shape + Clone + Display, U: Clone + Debug>(item1: &T, item2: &U) -> i32 {
    22
}

// it can be written

fn display_info<T, U>(item1: &T, item2: &U) -> i32
    where 
        T: Shape + Clone + Display,
        U: Clone + Debug
    {
        22
    }

// return type that impl specific trait

fn returns_shape() -> impl Shape {
    Circle {
        radius: 3.5,
    }
}
