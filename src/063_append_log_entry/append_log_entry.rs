use std::fs::OpenOptions;
use std::io::{self, Write};
use chrono::Local;

fn append_log_entry(filename: &str, message: &str) -> io::Result<()> {
    // Open file in append mode (create if doesn't exist)
    let mut file = OpenOptions::new()
        .append(true)
        .create(true)
        .open(filename)?;

    // Get current timestamp
    let timestamp = Local::now().format("%Y-%m-%d %H:%M:%S");
    
    // Write log entry
    writeln!(file, "[{}] {}", timestamp, message)?;
    
    Ok(())
}

fn main() -> io::Result<()> {
    let log_file = "app.log";
    
    println!("Logging to: {}", log_file);
    
    // Append some log entries
    append_log_entry(log_file, "Application started")?;
    append_log_entry(log_file, "Performing operation 1")?;
    append_log_entry(log_file, "Operation completed successfully")?;
    
    println!("Logged 3 entries to {}", log_file);
    
    // Display the log file contents
    println!("\nLog contents:");
    let contents = std::fs::read_to_string(log_file)?;
    println!("{}", contents);
    
    Ok(())
}