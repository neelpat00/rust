// First project that just does the conversion of temperature between 
// Fahrenheit and Celcius

use std::io;

fn fahrenheit_to_celcius(x: f64) -> f64 {
    (x - 32.0) * (5.0/9.0)
}

fn describe_temp(x: f64) -> &'static str {
    if x < 10.0 { "cold" } else if x < 25.0 { "nice" } else { "hot" }
}

fn main() {
    println!("Input the temperature in Fahrenheit: ");
    let mut temp = String::new();
    
    // Code that read the user's input'
    io::stdin()
        .read_line(&mut temp)
        .expect("Failed to read line!");
    
    // Converts the string into f64 type
    let temp: f64 = temp
        .trim()
        .parse()
        .expect("Please type a number!");
    
    
    let temp_in_c = fahrenheit_to_celcius(temp);
    println!("{temp} F to Celsius is {temp_in_c}");
    let descp = describe_temp(temp_in_c);
    println!("The temperature of {temp:.1} F is {temp_in_c:.1} which is {descp}!")
    
}
