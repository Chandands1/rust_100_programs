fn count_words(s: &str) -> usize {
    s.split_whitespace().count()
}

fn main() {
    let text = "Rust is a systems programming language";
    println!("Text: {}", text);
    println!("Word count: {}", count_words(text));
}