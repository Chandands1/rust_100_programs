fn main() {
    // Create (Initialize)
    let mut fruits: Vec<&str> = vec!["apple", "banana"];
    println!("Initial vector: {:?}", fruits);

    // Create (Add elements)
    fruits.push("orange");
    fruits.push("grape");
    println!("After push: {:?}", fruits);

    // Read (Access elements)
    println!("Second fruit: {}", fruits[1]);
    match fruits.get(4) {
        Some(fruit) => println!("Fifth fruit: {}", fruit),
        None => println!("No fifth fruit exists"),
    }

    // Update (Modify elements)
    fruits[0] = "kiwi";
    println!("After update: {:?}", fruits);

    // Delete (Remove elements)
    let popped = fruits.pop();
    println!("Popped: {:?}", popped);
    println!("After pop: {:?}", fruits);

    // Iterate
    println!("\nAll fruits:");
    for (i, fruit) in fruits.iter().enumerate() {
        println!("{}: {}", i, fruit);
    }

    // Clear
    fruits.clear();
    println!("\nAfter clear: {:?}", fruits);
}