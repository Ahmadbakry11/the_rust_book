use std::io;

fn main() {
    println!("===============Temperature degree converter===============");
    println!("===============Convert between Celsius, Kelvin and Fahrenheit===============");
    
    loop {
        let input_degree_type = loop {
            println!("Convert from ?");
            println!("Type K for Kelvin, C for Celsius and F for Fahrenheit");
            
            let mut input_degree_type = String::new();

            io::stdin()
                .read_line(&mut input_degree_type)
                .expect("Failed to read input temperature unit");
    
            match input_degree_type.trim().to_lowercase().as_str() {
                "c" | "k" | "f" => break input_degree_type.trim().to_lowercase(),

                _ => {
                    println!("Invalid temperature unit!");
                    continue;
                }
            }
                
        };

        let output_degree_type = loop {
            println!("Convert to ?"); 
            println!("Type K for Kelvin, C for Celsius and F for Fahrenheit");

            let mut output_degree_type= String::new();

            io::stdin()
                .read_line(&mut output_degree_type)
                .expect("Failed to read output temperature unit");

            match output_degree_type.trim().to_lowercase().as_str() {
                "c" | "k" | "f" => break output_degree_type.trim().to_lowercase(),

                _ => {
                    println!("Invalid temperature unit!");
                    continue;
                }
            }


        };

        let input_degree: f64 = loop {
            println!("Please input the temperature degree you want to convert");

            let mut input_degree = String::new();

            io::stdin()
            .read_line(&mut input_degree)
            .expect("invalid temperature degree!");

            match input_degree.trim().parse::<f64>() {
                Ok(val) => break val,
                Err(_) => continue,
            };
        };


        println!("input degree is {}", input_degree);
        let output_degree: f64 = convert_degree(&input_degree_type, &output_degree_type, input_degree);

        println!("The converted temperature is {:.2}", output_degree);
        println!("=============================================================");
    }
    
}

fn convert_degree(input_type: &str, output_type: &str, input_degree: f64) -> f64 {
    let result: f64 = match(input_type, output_type, input_degree) {
        ("c", "f", input_degree) => celsius_to_fahrenheit(input_degree),
        ("c", "k", input_degree) => celsius_to_kelvin(input_degree),
        ("c", "c", input_degree) => input_degree,
        ("f", "c", input_degree) => fahrenheit_to_celsius(input_degree),
        ("f", "k", input_degree) => fahrenheit_to_kelvin(input_degree),
        ("f", "f", input_degree) => input_degree,
        ("k", "c", input_degree) => kelvin_to_celsius(input_degree),
        ("k", "f", input_degree) => kelvin_to_fahrenheit(input_degree),
        ("k", "k", input_degree) => input_degree,
        _ => unreachable!(),
    };

    result
}

fn celsius_to_fahrenheit(input_degree: f64) -> f64 {
    (input_degree * 1.8) + 32.0
}

fn celsius_to_kelvin(input_degree: f64) -> f64 {
    input_degree + 273.15
}

fn fahrenheit_to_celsius(input_degree: f64) -> f64 {
    (input_degree - 32.0) * 5.0/9.0
}

fn fahrenheit_to_kelvin(input_degree: f64) -> f64 {
    (input_degree + 459.67) * 5.0/9.0   
}

fn kelvin_to_celsius(input_degree: f64) -> f64 {
    input_degree - 273.15
}

fn kelvin_to_fahrenheit(input_degree: f64) -> f64 {
    (input_degree - 273.15) * 1.8 + 32.0    
}