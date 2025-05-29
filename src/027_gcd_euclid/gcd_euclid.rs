use std::io;

fn main() {
    // Get two numbers from user
    let a = get_input("Enter first number: ");
    let b = get_input("Enter second number: ");
    
    // Calculate GCD using Euclid's algorithm
    let gcd = euclid_gcd(a, b);
    
    // Print result
    println!("GCD of {} and {} is {}", a, b, gcd);
}

// Euclid's algorithm implementation (iterative)
fn euclid_gcd(mut a: u32, mut b: u32) -> u32 {
    while b != 0 {
        let temp = b;
        b = a % b;
        a = temp;
    }
    a
}

// Helper function to get positive integer input
fn get_input(prompt: &str) -> u32 {
    loop {
        println!("{}", prompt);
        let mut input = String::new();
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read input");
            
        match input.trim().parse() {
            Ok(num) if num > 0 => return num,
            Ok(_) => println!("Number must be positive!"),
            Err(_) => println!("Please enter a valid positive integer!"),
        }
    }
}