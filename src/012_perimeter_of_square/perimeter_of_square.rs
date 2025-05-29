use std::io;  // Import standard input/output library

fn main() {
    // Get side length from user
    let side = get_input("Enter the side length of the square: ");
    
    // Calculate perimeter (4 × side)
    let perimeter = 4.0 * side;
    
    // Display result with 2 decimal places
    println!("Perimeter of the square = {:.2}", perimeter);
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
            Ok(num) if num > 0.0 => return num,  // Return if valid and positive
            Ok(_) => println!("⚠️ Please enter a positive number!"),  // Negative number
            Err(_) => println!("⚠️ Please enter a valid number!"),  // Invalid format
        }
    }
}