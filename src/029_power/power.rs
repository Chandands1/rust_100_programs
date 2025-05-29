use std::io;

fn main() {
    // Get base and exponent from user
    let base = get_float_input("Enter base: ");
    let exponent = get_float_input("Enter exponent: ");
    
    // Calculate power
    let result = if exponent.fract() == 0.0 {
        // Integer exponent (more precise for whole numbers)
        power_int(base, exponent as i32)
    } else {
        // Floating-point exponent
        base.powf(exponent)
    };
    
    // Print result
    println!("{}^{} = {:.4}", base, exponent, result);
}

// Integer exponent version (more efficient for whole numbers)
fn power_int(base: f64, exp: i32) -> f64 {
    match exp {
        0 => 1.0,
        _ if exp < 0 => 1.0 / power_int(base, -exp),
        _ => {
            let mut result = 1.0;
            for _ in 0..exp {
                result *= base;
            }
            result
        }
    }
}

// Helper function to get numeric input
fn get_float_input(prompt: &str) -> f64 {
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