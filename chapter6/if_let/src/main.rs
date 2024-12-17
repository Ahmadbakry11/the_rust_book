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

fn main() {
    let mut max: u8 = 0;
    let max_config = Some(3u8);
    if let Some(inn_max) = max_config {
       max = inn_max;
    }

    println!("And the mutable max keeps value as {max}");

    let coins = [
        Coin::Dime, 
        Coin::Quarter(UsState::Alabama),
        Coin::Nickel,
        Coin::Quarter(UsState::Alaska),
        Coin::Penny
    ];
    
    let non_quarters = non_quarter_counts(&coins);
    let quarters = &coins.len() - non_quarters;

    println!("The coins have {quarters} quarters");
    println!("The coins have {} non quarters", non_quarters);
}

// create a function that returns the non quarters count from a coins Array of size 5

fn non_quarter_counts(coin_list: &[Coin;5]) -> usize {
    let mut count: usize = 0;

    for coin in coin_list {
        if let Coin::Quarter(state) = coin {
            println!("The coin is minted for {state:?}");
        }else {
            count += 1;
        }
    }

    count
}
