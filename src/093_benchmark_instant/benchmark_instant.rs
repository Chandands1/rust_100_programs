// Add `instant = "0.1"` to your Cargo.toml dependencies
// [dependencies]
// instant = "0.1"

use instant::Instant; // Import the Instant struct from the instant crate
use std::thread;      // Import the thread module for sleeping
use std::time::Duration; // Import Duration for sleep

fn main() {
    println!("Starting benchmark...");

    // Record the current time
    let start_time = Instant::now();

    // Simulate some work
    thread::sleep(Duration::from_millis(1500)); // Sleep for 1.5 seconds

    // Record the time after the work is done
    let end_time = Instant::now();

    // Calculate the duration difference
    let duration = end_time.duration_since(start_time);

    // Print the elapsed time
    println!("Work took: {:?}", duration);
    println!("Work took: {} milliseconds.", duration.as_millis());
}