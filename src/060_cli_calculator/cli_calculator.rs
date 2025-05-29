use std::env;
use std::process;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() != 4 {
        eprintln!("Usage: {} <num1> <operator> <num2>", args[0]);
        eprintln!("Operators: +, -, *, /");
        process::exit(1);
    }

    let num1: f64 = match args[1].parse() {
        Ok(n) => n,
        Err(_) => {
            eprintln!("Error: First argument must be a number");
            process::exit(1);
        }
    };

    let operator = &args[2];

    let num2: f64 = match args[3].parse() {
        Ok(n) => n,
        Err(_) => {
            eprintln!("Error: Third argument must be a number");
            process::exit(1);
        }
    };

    let result = match operator.as_str() {
        "+" => num1 + num2,
        "-" => num1 - num2,
        "*" => num1 * num2,
        "/" => {
            if num2 == 0.0 {
                eprintln!("Error: Division by zero");
                process::exit(1);
            }
            num1 / num2
        }
        _ => {
            eprintln!("Error: Invalid operator. Use +, -, *, /");
            process::exit(1);
        }
    };

    println!("{} {} {} = {}", num1, operator, num2, result);
}