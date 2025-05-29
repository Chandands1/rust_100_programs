use std::env; // Import the environment module

fn main() {
    // ---
    // ## Reading a Specific Environment Variable
    // ---

    // Define the name of the environment variable we want to read
    let var_name = "MY_APP_SETTING";

    // Attempt to read the environment variable.
    // `env::var()` returns a `Result<String, VarError>`.
    // `Ok(value)` if the variable is found, `Err(VarError)` if not.
    match env::var(var_name) {
        Ok(val) => {
            // If the variable is found, print its value
            println!("The value of {}: {}", var_name, val);
        }
        Err(e) => {
            // If the variable is not found or there's another error, print an error message
            eprintln!("Couldn't read {}: {}", var_name, e);
            eprintln!("Hint: Try setting it, e.g., `export {}='some_value'` before running.", var_name);
        }
    }

    // ---
    // ## Reading Another Common Environment Variable (PATH)
    // ---

    // Read the PATH environment variable, which is usually present on most systems
    match env::var("PATH") {
        Ok(val) => {
            println!("\nThe PATH environment variable is: {}", val);
        }
        Err(e) => {
            eprintln!("\nCouldn't read PATH: {}", e);
        }
    }

    // ---
    // ## Iterating Through All Environment Variables
    // ---

    println!("\n--- All Environment Variables ---");
    // `env::vars()` returns an iterator over all environment variables.
    // Each item is a `Result<(String, String), VarError>`.
    for (key, value) in env::vars() {
        println!("{}: {}", key, value);
    }
    println!("---------------------------------");
}