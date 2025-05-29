use std::io;

fn main() {
    // Get first number from user
    let num1 = get_input("Enter first number: ");
    
    // Get second number from user
    let num2 = get_input("Enter second number: ");
    
    // Compare and find the largest number
    if num1 > num2 {
        println!("Largest number is: {}", num1);
    } else if num2 > num1 {
        println!("Largest number is: {}", num2);
    } else {
        println!("Both numbers are equal");
    }
}

// Helper function to get validated number input
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