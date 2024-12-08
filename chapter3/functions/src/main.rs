fn main() {
    println!("Hello, world!");
    another_function();
    print_label_measurement(5, 'h');
    println!("The ratio is {}", print_ratio_format(99));
    let y = {
        let x = 1;
        let z = x + 1;
        z + 97

    };

    // let arr: [i32; 3] = [1, 2, 3];

    // println!("The outband number is {}", arr[3]);

    println!("The y value is {}", y);

    let z = plus_one(8);
    println!("The incremented value is: {}", z);

    increment(-11);

    let sum: i32 = adder(9, 90);
    println!("The result is {}", sum);
    let arr = [-1, -2, -3, -4, -5, -6, -7, 8, 9, 10];
    let negative_sum = sum_negative(arr);
    println!("The sum of the first negative numbers is: {}", negative_sum);

    two_level_ladder();

    count_down(9);

    print_array(arr);

    print_collection(arr);

    range_count_down();
    /*
        Describing overflow and block comments at the same time.
    */ 

    // let a: u8 = 256;

    // println!("The overflow comes here: {}", a);

}

fn another_function(){
    println!("I am coming from the another_function");
}

fn print_label_measurement(unit: i32, unit_label: char) {
    println!("The measurement is calculated to be {unit} {unit_label}");
}

fn print_ratio_format(x: i32) -> String {
    let x = x.to_string();
    x + "%"
}

fn plus_one(x: i32) -> i32 {
    x + 1
}

fn increment(x: i32) -> i32 {
    let mut x = if x > 9 { -15 } else { -25 };

    loop {
        println!("The value of x now is: {}", x);

        if x >= 0 {
            break;
        }
        
        x = x + 1;
    }
    x
}

fn adder(x: i32, y: i32) -> i32 {
    println!("The value of x is: {x}");
    println!("The value of y is: {y}");
    x + y
}

fn sum_negative(arr: [i32; 10]) -> i32 {
    let mut x: usize = 0;
    let mut sum = 0;

    loop {
        if arr[x] < 0 {
            sum += arr[x];
            
            // here will return from the function
            if arr[x] == -5 {
                return sum;
            }

            x += 1;

        } else {
            sum *= -1;
            break;
        }
    };
    sum
}

fn two_level_ladder() {
    let mut vertical: i32 = 0;
    
    'counting_vertical: loop {
        let mut side: i32 = 10;

        println!("We are @ vertical step number: {vertical}");
        if vertical == 2 {
            break 'counting_vertical;
        }

        loop {
            println!("We are @ side step number: {side}"); 
            if side == 9 {
                break;
            }

            side -= 1;
        }

        vertical += 1;
    }
}

fn count_down(mut number: u32) {
    while number > 0 {
        println!("We are @ number {}", number);

        if number == 4 {
            println!("You can stop counting here @ number: {}", number);
            break;
        }

        number -= 1;
    }
} 

fn print_array(arr: [i32; 10]) {
    let mut i = 0;

    while i < arr.len() {
        println!("The element @ index {} is: {}", i, arr[i]);
        i += 1;
    }
}

fn print_collection(arr: [i32; 10]) {
    for element in arr {
        println!("The current element is {}", element);
    }
}

fn range_count_down() {
    for element in (1..=9).rev() {
        println!("The current element in the range is {}", element);
    }
}
