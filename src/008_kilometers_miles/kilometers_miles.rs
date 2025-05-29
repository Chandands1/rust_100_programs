use std::io;

fn main() {
    // Get distance in kilometers from user
    let kilometers = get_input("Enter distance in kilometers: ");
    
    // Convert to miles (1 km = 0.621371 miles)
    let miles = kilometers * 0.621371;
    
    // Print result with 2 decimal places
    println!("{:.2} km = {:.2} miles", kilometers, miles);
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