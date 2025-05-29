use std::io;  // Import standard input/output library

fn main() {
    // Get length from user
    let length = get_input("Enter the length of the rectangle: ");
    
    // Get width from user
    let width = get_input("Enter the width of the rectangle: ");
    
    // Calculate area (length × width)
    let area = length * width;
    
    // Display result with 2 decimal places
    println!("Area of the rectangle = {:.2}", area);
}

/// Helper function to get validated floating-point input
fn get_input(prompt: &str) -> f64 {
    loop {
        println!("{}", prompt);  // Show prompt message
        
        let mut input = String::new();  // Create mutable string for input
        
        // Read user input
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read input");  // Handle potential read errors
            
        // Try to convert input to f64
        match input.trim().parse() {
            Ok(num) => return num,  // Return if valid
            Err(_) => println!("⚠️ Please enter a valid number!"),  // Show error and retry
        }
    }
}