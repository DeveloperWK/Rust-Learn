use std::io;
fn convert_temperatures(){
    println!("Choose conversion:");
    println!("1. Fahrenheit to Celsius");
    println!("2. Celsius to Fahrenheit");
    let mut choice = String::new();
    io::stdin().read_line(&mut choice).expect("Failed to read line");
    let mut input = String::new();
   println!("Enter temperature:");
    io::stdin().read_line(&mut input).expect("Failed to read line");
    let temperature:f64 = input.trim().parse().expect("Please enter a valid number");
    match choice.trim(){
        "1"=>{
            let celsius = (temperature-32.0)*(5.0/9.0);
            println!("{}°F = {:.2}°C", temperature, celsius);
    },
        "2"=>{
            let fahrenheit = (temperature * 9.0 / 5.0) + 32.0;
            println!("{}°C = {:.2}°F", temperature, fahrenheit);
    },
    _ =>{
            println!("Invalid choice! Please enter 1 or 2");
    }
}}



fn main() {
    convert_temperatures();
}

