use std::io;

fn main() {
    // Get first number
    let mut a = get_input("Enter first number: ");
    
    // Get second number
    let mut b = get_input("Enter second number: ");
    
    // Print values before swapping
    println!("Before swap: a = {}, b = {}", a, b);
    
    // Swap values using a temporary variable
    let temp = a;
    a = b;
    b = temp;
    
    // Print values after swapping
    println!("After swap: a = {}, b = {}", a, b);
}

// Helper function to get number input
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