use std::io;

fn main() {
    // Get number from user
    let num = get_input("Enter an integer: ");
    
    // Reverse the number
    let reversed = reverse_integer(num);
    
    // Print result
    println!("Reversed number: {}", reversed);
}

// Function to reverse an integer
fn reverse_integer(mut n: i32) -> i32 {
    let mut reversed = 0;
    while n != 0 {
        reversed = reversed * 10 + n % 10;
        n /= 10;
    }
    reversed
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