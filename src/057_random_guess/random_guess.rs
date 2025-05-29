use rand::Rng;
use std::io;

fn main() {
    println!("Guess the number between 1 and 100!");
    
    let secret_number = rand::thread_rng().gen_range(1..=100);
    let mut attempts = 0;

    loop {
        println!("Please enter your guess:");
        
        let mut guess = String::new();
        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Please enter a valid number!");
                continue;
            }
        };

        attempts += 1;

        match guess.cmp(&secret_number) {
            std::cmp::Ordering::Less => println!("Too small!"),
            std::cmp::Ordering::Greater => println!("Too big!"),
            std::cmp::Ordering::Equal => {
                println!("You win! Found in {} attempts.", attempts);
                break;
            }
        }
    }
}