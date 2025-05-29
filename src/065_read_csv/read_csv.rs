use std::fs::File;
use std::io::{self, BufRead, BufReader};
use std::path::Path;

fn read_csv_lines(filename: &str) -> io::Result<()> {
    let path = Path::new(filename);
    let file = File::open(path)?;
    let reader = BufReader::new(file);

    for (i, line) in reader.lines().enumerate() {
        let line = line?;
        let fields: Vec<&str> = line.split(',').collect();
        
        println!("Line {}: {:?}", i + 1, fields);
    }

    Ok(())
}

fn main() {
    println!("CSV Reader");
    println!("----------");

    let mut filename = String::new();
    println!("Enter CSV file path:");
    io::stdin().read_line(&mut filename).expect("Failed to read input");

    match read_csv_lines(filename.trim()) {
        Ok(_) => println!("\nFinished reading file"),
        Err(e) => eprintln!("Error: {}", e),
    }
}