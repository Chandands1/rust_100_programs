use std::thread;      // Import the thread module for sleeping
use std::time::Duration; // Import Duration to specify sleep times

fn main() {
    println!("Starting timer...");

    // Sleep for 2 seconds
    // `Duration::new(seconds, nanoseconds)` creates a duration.
    // `thread::sleep()` pauses the current thread for the specified duration.
    thread::sleep(Duration::new(2, 0));
    println!("2 seconds have passed.");

    // Sleep for 500 milliseconds (0.5 seconds)
    thread::sleep(Duration::from_millis(500));
    println!("Another 0.5 seconds have passed.");

    // Sleep for 1 microsecond
    thread::sleep(Duration::from_micros(1));
    println!("1 microsecond has passed.");

    println!("Timer finished.");
}