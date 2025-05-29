use std::io;

fn main() {
    // Get range from user
    let start = get_input("Enter start of range (≥2): ");
    let end = get_input("Enter end of range: ");

    // Validate range
    if start > end {
        println!("Invalid range: start must be ≤ end");
        return;
    }

    println!("Prime numbers between {} and {}:", start, end);
    
    // Find and print primes in range
    let mut count = 0;
    for num in start..=end {
        if is_prime(num) {
            print!("{} ", num);
            count += 1;
        }
    }
    
    println!("\nFound {} primes in this range", count);
}

// Optimized prime check function
fn is_prime(n: u32) -> bool {
    if n <= 1 {
        return false;
    }
    if n <= 3 {
        return true;
    }
    if n % 2 == 0 || n % 3 == 0 {
        return false;
    }
    
    let mut i = 5;
    let mut w = 2;
    while i * i <= n {
        if n % i == 0 {
            return false;
        }
        i += w;
        w = 6 - w; // Alternate between 2 and 4 (6-2=4, 6-4=2)
    }
    true
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
            Ok(num) => return num,
            Err(_) => println!("Please enter a positive integer!"),
        }
    }
}