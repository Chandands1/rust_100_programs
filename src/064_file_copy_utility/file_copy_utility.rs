use std::fs;
use std::io;
use std::path::Path;

fn copy_file(source: &str, destination: &str) -> io::Result<()> {
    let source_path = Path::new(source);
    let dest_path = Path::new(destination);

    // Verify source exists
    if !source_path.exists() {
        return Err(io::Error::new(
            io::ErrorKind::NotFound,
            format!("Source file '{}' not found", source),
        ));
    }

    // Verify source is a file
    if !source_path.is_file() {
        return Err(io::Error::new(
            io::ErrorKind::InvalidInput,
            format!("'{}' is not a file", source),
        ));
    }

    // Perform the copy
    fs::copy(source_path, dest_path)?;

    Ok(())
}

fn main() {
    println!("File Copy Utility");
    println!("----------------");

    let mut source = String::new();
    let mut dest = String::new();

    println!("Enter source file path:");
    io::stdin().read_line(&mut source).expect("Failed to read input");

    println!("Enter destination path:");
    io::stdin().read_line(&mut dest).expect("Failed to read input");

    let source = source.trim();
    let dest = dest.trim();

    match copy_file(source, dest) {
        Ok(_) => println!("Successfully copied '{}' to '{}'", source, dest),
        Err(e) => eprintln!("Error: {}", e),
    }
}