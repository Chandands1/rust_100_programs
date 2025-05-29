use std::io;

fn main() {
    // Get first number
    let num1 = get_input("Enter first number: ");
    
    // Get operation choice
    let op = get_operation();
    
    // Get second number
    let num2 = get_input("Enter second number: ");
    
    // Perform calculation
    match op {
        '+' => println!("Result: {}", num1 + num2),
        '-' => println!("Result: {}", num1 - num2),
        '*' => println!("Result: {}", num1 * num2),
        '/' => {
            if num2 != 0.0 {
                println!("Result: {}", num1 / num2);
            } else {
                println!("Error: Division by zero!");
            }
        },
        _ => println!("Invalid operation!"),
    }
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

// Helper function to get operation choice
fn get_operation() -> char {
    loop {
        println!("Choose operation (+, -, *, /):");
        let mut input = String::new();
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read input");
            
        match input.trim().chars().next() {
            Some(c) if "+-*/".contains(c) => return c,
            _ => println!("Please enter a valid operation!"),
        }
    }
}