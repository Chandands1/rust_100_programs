use std::io;

fn main() {
    // Get number from user
    let num = get_input("Enter a number: ");
    
    // Check if number is even or odd
    if num % 2 == 0 {
        println!("{} is even", num);
    } else {
        println!("{} is odd", num);
    }
}

// Helper function to get validated number input
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