fn main() {
    let query = "second";
    let s = "This is the fisrt line
        and this is the second line
        and this is the third line";

    let mut result: Vec<&str> = Vec::new();
    
    let lines: Vec<&str> = s.split("\n").collect();
    
    
    for line in lines {
        if line.contains(query) {
            result.push(line);
        }
    }
    
    println!("The target text is found in the following lines");

    for res in &result {
        println!("{res}");
    }

    println!("The number of occurences is {}", result.len());
}