use std::{io::{self, Write}, thread, time::Duration};

fn main() {
    let total_steps = 50; // Define the total number of steps for the progress bar
    let bar_width = 50;  // Define the width of the progress bar itself (e.g., 50 characters)

    print!("Progress: ["); // Print the opening of the progress bar

    // Loop through each step
    for i in 0..=total_steps {
        // Calculate the number of '#' characters to display
        let progress_chars = (i as f32 / total_steps as f32 * bar_width as f32) as usize;
        // Calculate the number of '-' characters to display
        let remaining_chars = bar_width - progress_chars;

        // Print the filled part of the bar
        for _ in 0..progress_chars {
            print!("#");
        }
        // Print the empty part of the bar
        for _ in 0..remaining_chars {
            print!("-");
        }

        // Calculate and print the percentage
        let percentage = (i as f32 / total_steps as f32) * 100.0;
        print!("] {:.1}%\r", percentage); // `\r` moves the cursor to the beginning of the line

        // Flush the output to ensure it's displayed immediately
        io::stdout().flush().unwrap();

        // Simulate some work being done
        thread::sleep(Duration::from_millis(50));
    }

    println!("\nTask complete!"); // Print a newline after the progress bar is done
}