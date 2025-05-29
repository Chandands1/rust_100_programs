use std::io;

fn main() {
    // Get input from user
    let n = get_input("Enter term position (n >= 0): ");
    
    // Calculate Fibonacci number
    let fib = fibonacci(n);
    
    // Print result
    println!("Fibonacci({}) = {}", n, fib);
}

// Basic recursive Fibonacci implementation
fn fibonacci(n: u32) -> u32 {
    match n {
        0 => 0,
        1 => 1,
        _ => fibonacci(n - 1) + fibonacci(n - 2),
    }
}

// Helper function to get non-negative integer input
fn get_input(prompt: &str) -> u32 {
    loop {
        println!("{}", prompt);
        let mut input = String::new();
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read input");
            
        match input.trim().parse() {
            Ok(num) => return num,
            Err(_) => println!("Please enter a non-negative integer!"),
        }
    }
}