use std::collections::HashMap;
use std::io;

fn main() {
    // Create memoization cache
    let mut memo: HashMap<u64, u64> = HashMap::new();
    
    // Get input from user
    let n = get_input("Enter term position (n > 0): ");
    
    // Calculate Fibonacci number
    let fib = fibonacci(n, &mut memo);
    
    // Print result
    println!("Fibonacci term {} = {}", n, fib);
}

// Recursive Fibonacci with memoization
fn fibonacci(n: u64, memo: &mut HashMap<u64, u64>) -> u64 {
    if n == 0 {
        return 0;
    }
    if n == 1 {
        return 1;
    }
    
    // Check if already calculated
    if let Some(&val) = memo.get(&n) {
        return val;
    }
    
    // Calculate and store result
    let res = fibonacci(n - 1, memo) + fibonacci(n - 2, memo);
    memo.insert(n, res);
    res
}

// Helper function to get validated positive integer
fn get_input(prompt: &str) -> u64 {
    loop {
        println!("{}", prompt);
        let mut input = String::new();
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read input");
            
        match input.trim().parse() {
            Ok(num) if num > 0 => return num,
            Ok(_) => println!("Number must be positive!"),
            Err(_) => println!("Please enter a valid positive integer!"),
        }
    }
}