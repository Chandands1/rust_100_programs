use std::io;

fn main() {
    // Get input number from user
    let num = get_input("Enter a number to generate its multiplication table: ");
    
    // Get table range from user
    let range = get_input("Enter how many multiples to generate: ");
    
    // Generate and print multiplication table
    println!("\nMultiplication Table for {}:", num);
    for i in 1..=range {
        println!("{} × {} = {}", num, i, num * i);
    }
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
            Ok(_) => println!("Please enter a positive number!"),
            Err(_) => println!("Please enter a valid positive integer!"),
        }
    }
}