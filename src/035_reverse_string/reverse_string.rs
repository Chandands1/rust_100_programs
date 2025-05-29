fn main() {
    // Original string
    let s = "Hello, world!";
    
    // Reverse the string using `chars().rev()`
    let reversed: String = s.chars().rev().collect();
    
    // Print the reversed string
    println!("Original: {}", s);
    println!("Reversed: {}", reversed);
}