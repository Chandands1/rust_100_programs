use std::io;

fn main() {
    // Get number from user
    let n = get_input("Enter a positive integer (0-20): ");
    
    // Calculate factorial using recursion
    let result = factorial(n);
    
    // Print result
    println!("{}! = {}", n, result);
}

// Recursive factorial function
fn factorial(n: u64) -> u64 {
    if n <= 1 {
        1
    } else {
        n * factorial(n - 1)
    }
}

// Helper function to get validated input (0-20 to prevent overflow)
fn get_input(prompt: &str) -> u64 {
    loop {
        println!("{}", prompt);
        let mut input = String::new();
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read input");
            
        match input.trim().parse() {
            Ok(num) if num <= 20 => return num,
            Ok(_) => println!("Number must be between 0 and 20!"),
            Err(_) => println!("Please enter a valid positive integer!"),
        }
    }
}