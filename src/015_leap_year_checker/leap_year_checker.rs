use std::io;

fn main() {
    // Get year from user
    let year = get_input("Enter a year: ");
    
    // Check if leap year
    let is_leap = if (year % 4 == 0 && year % 100 != 0) || year % 400 == 0 {
        true
    } else {
        false
    };
    
    // Print result
    println!("{} is {}a leap year", year, if is_leap { "" } else { "not " });
}

// Helper function to get validated year input
fn get_input(prompt: &str) -> i32 {
    loop {
        println!("{}", prompt);
        let mut input = String::new();
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read input");
            
        match input.trim().parse() {
            Ok(num) if num > 0 => return num,
            Ok(_) => println!("Year must be positive!"),
            Err(_) => println!("Please enter a valid year!"),
        }
    }
}