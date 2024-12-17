use std::io;

enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(UsState),
}
#[derive(Debug)]
enum UsState {
    Alabama,
    Alaska,
}

impl UsState {
   fn as_str(&self) -> &str {
        match self {
            Self::Alabama => "Alabama",
            Self::Alaska => "Alaska",
        }
   } 
}

fn main() {
    let coin = Coin::Penny;
    let quarter_coin = Coin::Quarter(UsState::Alabama);
    println!("The coin has {} cents", value_in_cents(quarter_coin));

    let init_num = increment(Some(1));

    println!("The incremented num is {}", init_num);

    let number = dublicate(Some(9));
    let dublicated_num = number.unwrap_or(1);
    println!("The number after dublication is {dublicated_num}");


    // Patterns That Bind to Values
    // let num
    let mut count: i32 = 0;

    loop {
        println!("Please throw the dice and type number!");
    
        let mut user_roll = String::new();
    
        io::stdin()
            .read_line(&mut user_roll)
            .expect("invalid input");
    
        let user_roll: u8 = match user_roll.trim().parse() {
            Ok(n) => n,
            _ => continue,
        };
    
        println!("You threw {}", user_roll);
    
        if out_of_range(user_roll) {
            println!("Number out of range. Try again!");
            continue;
        }
    
        match user_roll {
            1 => {
                win_fancy_hat();
                count += 1;
            },
            5 => {
                lose_fancy_hat();
                count -= 1;
            },
            _ => {
                println!("Your opponent's turn!");
            },
        }
    
        println!("Count is {count}");
        if count <= 0 {
            break;
        }
    }
    


}

fn value_in_cents(coin: Coin) -> u8 {
    match coin {
        Coin::Penny => 1,  // no need to add {} for short code, one line.
        Coin::Nickel => {  // add {} for multiple lines
            println!("Lucky Nickel");
            5
        },
        Coin::Dime => 10,
        Coin::Quarter(state) => {
            // we can define a method as_str and call it to print the state
            // or use debug
            println!("The Quarter state is: {}", {state.as_str()});
            println!("The Quarter state is: {state:?}");
            25
        }
    }
}

fn increment(num: Option<i32>) -> i32 {
    match num {
        Some(i) => i + 1,
        None => 0,
    }
}

fn dublicate(num: Option<i32>) -> Option<i32> {
    match num {
        None => None,
        Some(i) => Some(i * 2),
    }
}


fn roll_dice(mut num: u8) -> u8 {
    match num {
        1 => {
            win_fancy_hat();  // roll again
            1
        },
        5 => {
            lose_fancy_hat();  // roll again
            5
        },
        _ => {
            println!("You won a fancy hat!");
            0 // time for the other player to roll
        },
    }
}

fn win_fancy_hat() {
    println!("You won a fancy hat!");
}

fn lose_fancy_hat() {
    println!("You lost the fancy hat!");
}

fn move_forward(num: u8) -> u8 {
    roll_dice(num)
}

fn out_of_range(num: u8) -> bool {
    num < 1 || num > 6
}