use std::io;

fn main() {
    let principal = get_input("Enter the principal amount (P): ");
    let rate = get_input("Enter the annual interest rate (R %): ");
    let time = get_input("Enter the time in years (T): ");

    let simple_interest = (principal * rate * time) / 100.0;
    let total_amount = principal + simple_interest;

    println!("-------------------------------");
    println!("Simple Interest = {:.2}", simple_interest);
    println!("Total Amount = {:.2}", total_amount);
}

fn get_input(prompt: &str) -> f64 {
    loop {
        println!("{}", prompt);
        let mut input = String::new();
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read input");

        match input.trim().parse() {
            Ok(num) => return num,
            Err(_) => println!("⚠️ Please enter a valid number!"),
        }
    }
}