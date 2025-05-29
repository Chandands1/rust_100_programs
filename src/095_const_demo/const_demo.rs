// Define a compile-time constant
// `const` variables must have their type annotated.
// They are immutable and their value is embedded directly at compile time.
const MAX_RETRIES: u32 = 3;
const APPLICATION_NAME: &str = "My Awesome App";
const PI: f64 = 3.14159265359;

fn main() {
    println!("Application: {}", APPLICATION_NAME);
    println!("Maximum retries allowed: {}", MAX_RETRIES);

    let radius = 5.0;
    let area = PI * radius * radius;
    println!("Area of a circle with radius {}: {}", radius, area);

    // Constants cannot be changed after declaration
    // MAX_RETRIES = 5; // This would cause a compile-time error
}