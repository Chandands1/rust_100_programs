use serde::{Serialize, Deserialize};
use serde_json::{self, json};

#[derive(Debug, Serialize, Deserialize)]
struct Person {
    name: String,
    age: u8,
    skills: Vec<String>,
    is_active: bool,
}

fn main() {
    // Method 1: Using json! macro for quick JSON creation
    let quick_json = json!({
        "name": "Alice",
        "age": 28,
        "skills": ["Rust", "Python"],
        "is_active": true,
        "metadata": {
            "created_at": "2023-01-01",
            "modified_at": null
        }
    });

    println!("Quick JSON:\n{}\n", serde_json::to_string_pretty(&quick_json).unwrap());

    // Method 2: Serializing a struct
    let person = Person {
        name: "Bob".to_string(),
        age: 32,
        skills: vec!["JavaScript".to_string(), "TypeScript".to_string()],
        is_active: false,
    };

    let person_json = serde_json::to_string_pretty(&person).unwrap();
    println!("Struct JSON:\n{}\n", person_json);

    // Method 3: Serializing to a file
    let file_path = "person.json";
    std::fs::write(file_path, person_json).unwrap();
    println!("JSON written to {}", file_path);
}


// [dependencies]
// serde = { version = "1.0", features = ["derive"] }
// serde_json = "1.0"