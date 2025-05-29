fn is_palindrome(s: &str) -> bool {
    // Normalize the string (lowercase and filter non-alphanumeric)
    let normalized: String = s
        .chars()
        .filter(|c| c.is_alphanumeric())
        .map(|c| c.to_ascii_lowercase())
        .collect();
    
    // Compare with its reverse
    normalized == normalized.chars().rev().collect::<String>()
}

fn main() {
    let test_cases = ["racecar", "hello", "A man, a plan, a canal: Panama", "12321"];
    
    for &s in &test_cases {
        println!("'{}' is a palindrome: {}", s, is_palindrome(s));
    }
}