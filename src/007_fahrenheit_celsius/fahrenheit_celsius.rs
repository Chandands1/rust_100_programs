use std::io;

fn main() {
    // Get temperature in Fahrenheit from user
    let fahrenheit = get_input("Enter temperature in Fahrenheit: ");
    
    // Convert to Celsius
    let celsius = (fahrenheit - 32.0) * 5.0 / 9.0;
    
    // Print result with 1 decimal place
    println!("{:.1}°F is {:.1}°C", fahrenheit, celsius);
}

// Helper function to get validated number input
fn get_input(prompt: &str) -> f64 {
    loop {
        println!("{}", prompt);
        let mut input = String::new();
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read input");
            
        match input.trim().parse() {
            Ok(num) => return num,
            Err(_) => println!("Please enter a valid number!"),
        }
    }
}