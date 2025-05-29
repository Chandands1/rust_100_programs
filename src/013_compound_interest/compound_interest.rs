use std::io;

fn main() {
    // Get principal amount
    let principal = get_input("Enter principal amount: ");
    
    // Get annual interest rate (as percentage)
    let rate = get_input("Enter annual interest rate (%): ");
    
    // Get time in years
    let time = get_input("Enter time in years: ");
    
    // Get compounding frequency per year
    let n = get_input("Enter compounding frequency per year: ");
    
    // Calculate compound interest
    let amount = principal * (1.0 + (rate / 100.0) / n).powf(n * time);
    let interest = amount - principal;
    
    // Print results
    println!("Compound Interest: {:.2}", interest);
    println!("Total Amount: {:.2}", amount);
}

// Helper function to get validated positive number input
fn get_input(prompt: &str) -> f64 {
    loop {
        println!("{}", prompt);
        let mut input = String::new();
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read input");
            
        match input.trim().parse() {
            Ok(num) if num > 0.0 => return num,
            Ok(_) => println!("Please enter a positive number!"),
            Err(_) => println!("Please enter a valid number!"),
        }
    }
}