fn is_vowel_or_consonant(c: char) -> String {
    match c.to_ascii_lowercase() {
        'a' | 'e' | 'i' | 'o' | 'u' => format!("'{}' is a vowel", c),
        _ if c.is_ascii_alphabetic() => format!("'{}' is a consonant", c),
        _ => format!("'{}' is not a letter", c),
    }
}

fn main() {
    let letters = ['A', 'b', 'E', 'z', '1', '!', 'x'];
    
    for &c in &letters {
        println!("{}", is_vowel_or_consonant(c));
    }
}