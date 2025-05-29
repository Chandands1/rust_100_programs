use std::fs::OpenOptions; // Import OpenOptions for file manipulation
use std::io::Write;       // Import the Write trait for writing to files
use chrono::Local;        // Import Local from chrono for local time

fn main() {
    let log_file_name = "application.log"; // Define the name of the log file

    // Log a simple message
    log_message(log_file_name, "Application started successfully.");

    // Log another message after some hypothetical operation
    log_message(log_file_name, "Processing data...");

    // Log an error message
    log_message(log_file_name, "ERROR: Failed to connect to database.");

    println!("Log messages written to {}", log_file_name);
}

// Function to log a message to a specified file
fn log_message(file_name: &str, message: &str) {
    // Open the file in append mode.
    // `.create(true)` creates the file if it doesn't exist.
    // `.append(true)` ensures new writes are added to the end of the file.
    let mut file = OpenOptions::new()
        .create(true)
        .append(true)
        .open(file_name)
        .expect("Failed to open log file"); // Panic if the file cannot be opened

    // Get the current local date and time
    let now = Local::now();
    // Format the log entry: "[YYYY-MM-DD HH:MM:SS] Message"
    let log_entry = format!("[{}] {}\n", now.format("%Y-%m-%d %H:%M:%S"), message);

    // Write the log entry to the file.
    // `file.write_all()` returns a `Result`, so `expect()` handles potential errors.
    file.write_all(log_entry.as_bytes())
        .expect("Failed to write to log file");
}