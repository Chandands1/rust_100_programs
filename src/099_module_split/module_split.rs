// --- src/main.rs ---
// Declare a module named `greetings`.
// This tells Rust to look for `src/greetings.rs` or `src/greetings/mod.rs`.
mod greetings;

// Import the `hello` function from the `greetings` module.
// We can also use `use greetings::*` to import all public items.
use greetings::hello;

fn main() {
    println!("From main.rs:");
    // Call the function defined in the `greetings` module.
    hello();

    // Call another function from the `greetings` module using its full path.
    greetings::goodbye();
}

// --- src/greetings.rs ---
// (This file would be created in the same directory as src/main.rs)

// Make the `hello` function public so it can be accessed from `main.rs`.
pub fn hello() {
    println!("Hello from the greetings module!");
}

// Another public function
pub fn goodbye() {
    println!("Goodbye from the greetings module!");
}

// An internal (private) function within the module
fn internal_greeting() {
    println!("This is an internal greeting (private to greetings module).");
}