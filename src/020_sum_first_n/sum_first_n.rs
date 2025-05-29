use std::io;

fn main() {
    // Get input from user
    let n = get_input("Enter a positive integer (N): ");
    
    // Calculate sum
    let sum = sum_first_n(n);
    
    // Print result
    println!("Sum of first {} natural numbers = {}", n, sum);
}

// Recursive function to calculate sum of first N numbers
fn sum_first_n(n: u32) -> u32 {
    if n == 0 {
        0
    } else {
        n + sum_first_n(n - 1)
    }
}

// Helper function to get validated positive integer input
fn get_input(prompt: &str) -> u32 {
    loop {
        println!("{}", prompt);
        let mut input = String::new();
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read input");
            
        match input.trim().parse() {
            Ok(num) => return num,
            Err(_) => println!("Please enter a positive integer!"),
        }
    }
}