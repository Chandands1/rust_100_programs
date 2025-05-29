use std::io;

fn main() {
    // Get number from user
    let n = get_input("Enter a positive integer: ");
    
    // Calculate factorial
    let mut factorial = 1;
    for i in 1..=n {
        factorial *= i;
    }
    
    // Print result
    println!("{}! = {}", n, factorial);
}

// Helper function to get validated positive integer input
fn get_input(prompt: &str) -> u64 {
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