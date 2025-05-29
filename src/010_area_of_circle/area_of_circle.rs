use std::io;  // Import the standard I/O library for user input

fn main() {
    // Get the radius from the user
    let radius = get_input("Enter the radius of the circle: ");
    
    // Calculate the area using the formula: π × r²
    let area = std::f64::consts::PI * radius.powi(2);
    
    // Display the result (formatted to 2 decimal places)
    println!("Area of the circle = {:.2}", area);
}

/// Helper function to get a valid floating-point number from the user
/// - `prompt`: The message displayed to the user (e.g., "Enter radius...")
/// - Returns: A validated `f64` number
fn get_input(prompt: &str) -> f64 {
    loop {  // Keep asking until valid input is given
        println!("{}", prompt);  // Display the prompt
        
        let mut input = String::new();  // Store user input here
        
        // Read input from the user
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read input");  // Crash if input fails (unlikely)
        
        // Try to convert the input into an `f64` (floating-point number)
        match input.trim().parse() {
            Ok(num) => return num,  // Success → return the number
            Err(_) => println!("⚠️ Please enter a valid number!"),  // Error → retry
        }
    }
}