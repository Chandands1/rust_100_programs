use std::collections::HashMap;

fn word_frequency(text: &str) -> HashMap<String, u32> {
    let mut frequency = HashMap::new();
    
    for word in text.split_whitespace() {
        // Clean words by removing punctuation and converting to lowercase
        let cleaned_word = word.trim_matches(|c: char| !c.is_alphabetic())
                             .to_lowercase();
        
        if !cleaned_word.is_empty() {
            *frequency.entry(cleaned_word).or_insert(0) += 1;
        }
    }
    
    frequency
}

fn main() {
    let text = "Hello world, hello Rust! World of Rust is wonderful.";
    let frequency = word_frequency(text);
    
    println!("Word frequencies:");
    for (word, count) in &frequency {
        println!("{}: {}", word, count);
    }
    
    // Get specific word count
    println!("\nCount for 'rust': {}", frequency.get("rust").unwrap_or(&0));
}