fn count_vowels_and_consonants(s: &str) -> (usize, usize) {
    let mut vowels = 0;
    let mut consonants = 0;

    for c in s.chars() {
        match c.to_ascii_lowercase() {
            'a' | 'e' | 'i' | 'o' | 'u' => vowels += 1,
            'a'..='z' => consonants += 1,
            _ => continue,
        }
    }
    (vowels, consonants)
}

fn main() {
    let test_string = "Hello, Rust World!";
    let (vowels, consonants) = count_vowels_and_consonants(test_string);
    
    println!("String: {}", test_string);
    println!("Vowels: {}", vowels);
    println!("Consonants: {}", consonants);
}