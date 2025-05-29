use rand::Rng;
use std::io;

fn main() {
    println!("Dice Roll Simulator");
    println!("-------------------");

    let mut rng = rand::thread_rng();
    let mut total_rolls = 0;
    let mut results = [0; 6]; // Track counts for each face (1-6)

    loop {
        println!("\nOptions:");
        println!("1. Roll dice");
        println!("2. View statistics");
        println!("3. Exit");
        println!("Enter your choice:");

        let mut choice = String::new();
        io::stdin()
            .read_line(&mut choice)
            .expect("Failed to read input");

        match choice.trim() {
            "1" => {
                let roll = rng.gen_range(1..=6);
                println!("\nYou rolled: {}", roll);
                results[roll - 1] += 1;
                total_rolls += 1;
            }
            "2" => {
                println!("\nStatistics ({} total rolls):", total_rolls);
                for (i, &count) in results.iter().enumerate() {
                    let percentage = if total_rolls > 0 {
                        (count as f32 / total_rolls as f32) * 100.0
                    } else {
                        0.0
                    };
                    println!("{}: {} times ({:.1}%)", i + 1, count, percentage);
                }
            }
            "3" => {
                println!("Thanks for playing!");
                break;
            }
            _ => println!("Invalid choice, please try again."),
        }
    }
}