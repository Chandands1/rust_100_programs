use std::io;

fn main() {
    // Get student's score from user
    let score = get_input("Enter student's score (0-100): ");
    
    // Determine grade based on score
    let grade = match score {
        _ if score >= 90.0 => "A",
        _ if score >= 80.0 => "B",
        _ if score >= 70.0 => "C",
        _ if score >= 60.0 => "D",
        _ => "F",
    };
    
    // Print the grade
    println!("Grade: {}", grade);
}

// Helper function to get validated score input (0-100)
fn get_input(prompt: &str) -> f64 {
    loop {
        println!("{}", prompt);
        let mut input = String::new();
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read input");
            
        match input.trim().parse() {
            Ok(num) if (0.0..=100.0).contains(&num) => return num,
            Ok(_) => println!("Score must be between 0 and 100!"),
            Err(_) => println!("Please enter a valid number!"),
        }
    }
}