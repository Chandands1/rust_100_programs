use std::fs;
use std::io;
use std::path::Path;

fn read_file_contents(filename: &str) -> io::Result<String> {
    let path = Path::new(filename);
    
    // Verify the file exists and is readable
    if !path.exists() {
        return Err(io::Error::new(
            io::ErrorKind::NotFound,
            format!("File '{}' not found", filename),
        ));
    }

    // Check if it's a file (not a directory)
    if !path.is_file() {
        return Err(io::Error::new(
            io::ErrorKind::InvalidInput,
            format!("'{}' is not a file", filename),
        ));
    }

    // Read the file contents
    fs::read_to_string(path)
}

fn main() {
    println!("Enter file path to read:");
    let mut filename = String::new();
    
    io::stdin()
        .read_line(&mut filename)
        .expect("Failed to read input");

    let filename = filename.trim();

    match read_file_contents(filename) {
        Ok(contents) => {
            println!("\nFile contents:\n{}", contents);
        }
        Err(e) => {
            eprintln!("\nError reading file '{}': {}", filename, e);
        }
    }
}