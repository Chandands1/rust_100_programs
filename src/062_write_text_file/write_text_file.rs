use std::fs::{self, File};
use std::io::{self, Write};
use std::path::Path;

enum FileMode {
    Write,
    Append,
}

fn write_to_file(filename: &str, content: &str, mode: FileMode) -> io::Result<()> {
    let path = Path::new(filename);
    
    match mode {
        FileMode::Write => {
            // Create new file or overwrite existing
            let mut file = File::create(path)?;
            file.write_all(content.as_bytes())?;
        }
        FileMode::Append => {
            // Open file in append mode or create if doesn't exist
            let mut file = fs::OpenOptions::new()
                .append(true)
                .create(true)
                .open(path)?;
            writeln!(file, "{}", content)?;
        }
    }
    
    Ok(())
}

fn main() -> io::Result<()> {
    let filename = "example.txt";
    
    // Write mode (creates new file or overwrites)
    write_to_file(filename, "This is the first line.", FileMode::Write)?;
    println!("Successfully wrote to file");
    
    // Append mode (adds to existing file)
    write_to_file(filename, "This line was appended.", FileMode::Append)?;
    println!("Successfully appended to file");
    
    // Read and display the file
    let contents = fs::read_to_string(filename)?;
    println!("\nFile contents:\n{}", contents);
    
    Ok(())
}