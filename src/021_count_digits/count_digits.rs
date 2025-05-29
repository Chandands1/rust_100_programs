use std::io;

fn main() {
    // Get number from user
    let num = get_input("Enter a number: ");
    
    // Count digits
    let count = if num == 0 {
        1  // Special case for 0
    } else {
        count_digits(num.abs())
    };
    
    // Print result
    println!("Number of digits: {}", count);
}

// Function to count digits (recursive approach)
fn count_digits(n: i32) -> u32 {
    if n == 0 {
        0
    } else {
        1 + count_digits(n / 10)
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