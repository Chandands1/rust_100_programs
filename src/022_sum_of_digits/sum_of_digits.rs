use std::io;

fn main() {
    // Get number from user
    let num = get_input("Enter a number: ");
    
    // Calculate sum of digits
    let sum = sum_digits(num.abs());
    
    // Print result
    println!("Sum of digits: {}", sum);
}

// Function to calculate sum of digits (recursive approach)
fn sum_digits(n: i32) -> i32 {
    if n == 0 {
        0
    } else {
        (n % 10) + sum_digits(n / 10)
    }
}

// Helper function to get integer input
fn get_input(prompt: &str) -> i32 {
    loop {
        println!("{}", prompt);
        let mut input = String::new();
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read input");
            
        match input.trim().parse() {
            Ok(num) => return num,
            Err(_) => println!("Please enter a valid integer!"),
        }
    }
}