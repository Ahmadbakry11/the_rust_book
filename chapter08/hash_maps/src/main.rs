use std::collections::HashMap;

fn main() {
    let mut scores: HashMap<&String, u32> = HashMap::new();

    let blue_team = String::from("Blue");
    let yellow_team = String::from("Yello");

    scores.insert(&blue_team, 99);
    scores.insert(&yellow_team, 9);

    let blue_score = scores.get(&blue_team).copied().unwrap_or(0);
    /*
        scores.get(&blue_team) returns an option<&u32> and we call copied
        to return an option<i32>
    */ 
    println!("The Blue Team has a score of {}", blue_score);

    // iterate over a HashMap

    for (key, value ) in &scores {
        println!("The {key} team has a score of {value} points", )
    }

    /*
        HashMaps and ownership
        Values that implement the Copy trait will be copied and values that
        do not implement the copy trait, the ownership will be moved to the hash map

        for example the ownership of values coffee will be
        moved to the shopping_cart hash_map after calling insert
        so, we will not be able to use coffee value any more.
        On the opposite side, the coffe_price implement the copy trait and its value
        ownership will not be moved.
    */

    let mut shopping_cart:HashMap<String, f64> = HashMap::new();
    
    let coffee = String::from("Coffee");
    let coffe_price = 99.9;

    shopping_cart.insert(coffee,coffe_price);
    
    // the below code will not compile.
    // println!("The {} price is {}", coffee, coffe_price);

    /*
        Updating the HashMap
        =========================
        If we insert value to an existing key with its old value
        the new value will replace the old value
    */ 

    scores.insert(&blue_team, 199);
    scores.insert(&blue_team, 299);
    
    let updated_blue_score = scores.get(&blue_team).copied().unwrap_or(0);
    println!("The updated score of the blue team is {updated_blue_score}");  //299 replaces 199

    // Adding key value pair only if a key does not exist.
    /*
        Using the entry method, which will return an Enum of type Entry
        and then calling the .or_insert on that entry will return a mutable ref to that value
        if it exists or inserts the value if the key does not exist

        now scores has values
        {"Blue": 299 , "Yello": 9}
        lets try to upadte the yellow score with .or_insert and inserting a new team using
        the same method
    */ 
    
    let red_team = String::from("Red");
    scores.entry(&red_team).or_insert(7);
    scores.entry(&yellow_team).or_insert(77);

    /*
        Now scores have:
        {"Blue": 299 , "Yello": 9, "Red": 7}
    */ 

    println!("The scores HashMap now has the following scores: {scores:?}");
    // {"Yello": 9, "Red": 7, "Blue": 299}
    
    /*
        Updating a Value Based on the Old Value
        ===============================================
    */ 
    let text = "hello world wonderful world";
    let mut word_counter:HashMap<&str, usize> = HashMap::new();

    for word in text.split_whitespace() {
        let count = word_counter.entry(word).or_insert(0);
        // note that count is a mutable refernce to the value of the existing key or the new value
        // so, you need to derefernce it as below
        *count += 1;
    }

    println!("The words counter now contains: {word_counter:?}");

    // The words counter now contains: {"hello": 1, "world": 2, "wonderful": 1}

}
