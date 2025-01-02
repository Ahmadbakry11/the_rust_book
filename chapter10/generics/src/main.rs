mod generic_types;

struct Point<T> {
    x: T,
    y: T,
}

impl<T> Point<T> {
    fn x(&self) -> &T {
        &self.x
    }

    fn y(&self) -> &T {
        &self.y
    }
}

impl Point<f32> {
    fn distance_from_origin(&self) -> f32 {
        (self.x().powi(2) + self.y().powi(2)).sqrt()
    }
}

struct WidePoint<T, U> {
    x: T, 
    y: U,
}


struct MixPoint<X, Y> {
    x: X,
    y: Y,
}

impl<X, Y> MixPoint<X, Y> {
    fn mix_up<N, K>(self, other: MixPoint<N, K>) -> MixPoint<X, K> {
        MixPoint {
            x: self.x,
            y: other.y
        }
    }
}

fn main() {
    let numbers_list = vec![9,3,1,18,23,0,11];

    let mut largest = numbers_list[0];

    for num in &numbers_list {
        if *num > largest {
            largest = *num;
        }
    }

    println!("The largest number is {largest}");
    
    let list = vec![102, 34, 6000, 89, 54, 2, 43, 8];

    println!("The largest number in this list is {}", largest_number(&list));

    let char_list = ['a', 'b', 'c', 'd'];

    println!("Largest char is {}", generic_types::largest_item(&char_list));

    println!("Largest number is {}", generic_types::largest_item(&list));

    // structs and generic types

    let p1 = Point { x: 10, y: 9};
    let p2 = Point { x: 'c', y: 'd'};
    
    let x_value = p1.x();  //it is a ref
    let y_value = p1.y();

    let sum = x_value + y_value;
    // println!("The coordinations are {} and {}", p1.x(), p1.y());

    let p3 = WidePoint { x: 99, y: "y-axis" };
    let p4 = WidePoint { x: 'x', y: 6.5 };
    let p5: Point<f32> = Point { x: 2.2, y: 6.5 };

    println!("The distance from origin is {}", p5.distance_from_origin());

    let p6 = MixPoint { x: 'H', y: 9.9};
    let other_point = MixPoint {x: 99, y: "Hello"};

    let new_point = p6.mix_up(other_point);

    println!("The new point is {}, {}", new_point.x, new_point.y);
    
}


fn largest_number(list: &[i32]) -> &i32 {
    let mut largest = &list[0];

    for num in list {
        if num > largest {
            largest = num;
        }
    }

    largest
}