use std::io;

fn main() {
    // Get character from user
    let c = get_char_input("Enter a character: ");
    
    // Get ASCII value
    let ascii = c as u8;
    
    // Print result
    println!("ASCII value of '{}' = {}", c, ascii);
}

// Helper function to get single character input
fn get_char_input(prompt: &str) -> char {
    loop {
        println!("{}", prompt);
        let mut input = String::new();
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read input");
            
        match input.trim().chars().next() {
            Some(c) => return c,
            None => println!("Please enter a valid character!"),
        }
    }
}