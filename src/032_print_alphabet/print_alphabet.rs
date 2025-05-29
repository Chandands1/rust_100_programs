fn main() {
    // Loop through ASCII values from 'A' to 'Z'
    for c in b'A'..=b'Z' {
        print!("{} ", c as char);
    }
    println!(); // Add a newline at the end
}