use std::io;

fn main() {
    // Get number from user
    let num = get_input("Enter a number: ");
    
    // Check if palindrome
    if is_palindrome(num) {
        println!("{} is a palindrome", num);
    } else {
        println!("{} is not a palindrome", num);
    }
}

// Function to check if a number is palindrome
fn is_palindrome(n: i32) -> bool {
    if n < 0 {
        return false; // Negative numbers can't be palindromes
    }
    
    let mut original = n;
    let mut reversed = 0;
    
    while original > 0 {
        reversed = reversed * 10 + original % 10;
        original /= 10;
    }
    
    n == reversed
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