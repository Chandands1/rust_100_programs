use serde::{Deserialize, Serialize};
use std::error::Error;

#[derive(Debug, Serialize, Deserialize)]
struct User {
    id: u32,
    name: String,
    email: String,
    active: bool,
    skills: Vec<String>,
    metadata: Option<serde_json::Value>,
}

fn main() -> Result<(), Box<dyn Error>> {
    // Sample JSON string
    let json_data = r#"
        {
            "id": 42,
            "name": "Alice",
            "email": "alice@example.com",
            "active": true,
            "skills": ["Rust", "Python"],
            "metadata": {
                "created_at": "2023-01-01",
                "admin": false
            }
        }
    "#;

    // Parse JSON into User struct
    let user: User = serde_json::from_str(json_data)?;

    println!("{:#?}", user);

    // Access fields
    println!("\nUser Details:");
    println!("ID: {}", user.id);
    println!("Name: {}", user.name);
    println!("Email: {}", user.email);
    println!("Active: {}", user.active);
    println!("Skills: {}", user.skills.join(", "));

    if let Some(meta) = user.metadata {
        println!("\nMetadata:");
        println!("{}", meta);
    }

    Ok(())
}

// [dependencies]
// serde = { version = "1.0", features = ["derive"] }
// serde_json = "1.0"