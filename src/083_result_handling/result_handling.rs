fn main() {
    let numerator = 10;
    let denominator = 0; // This will cause a divide-by-zero error if not handled

    // Attempt to divide, handling the potential error
    let result = divide(numerator, denominator);

    // Match on the Result to handle success or error
    match result {
        Ok(value) => println!("Result: {}", value), // Division successful
        Err(e) => eprintln!("Error: {}", e), // Division failed
    }
}

// Function that attempts to divide two integers, returning a Result
fn divide(numerator: i32, denominator: i32) -> Result<i32, String> {
    if denominator == 0 {
        // Return an Err if the denominator is zero
        Err("Cannot divide by zero".to_string())
    } else {
        // Return an Ok with the result of the division
        Ok(numerator / denominator)
    }
}