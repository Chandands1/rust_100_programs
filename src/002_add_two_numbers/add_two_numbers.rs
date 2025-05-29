use std::io;

fn main() {
    // Get first number from user
    let num1 = get_input("Enter first number: ");
    
    // Get second number from user
    let num2 = get_input("Enter second number: ");
    
    // Calculate sum
    let sum = num1 + num2;
    
    // Print result
    println!("Sum = {}", sum);
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