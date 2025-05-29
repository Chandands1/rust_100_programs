// This example typically goes in src/main.rs or src/lib.rs.
// To use feature flags, you need to define them in your Cargo.toml:
// [features]
// my_feature = []
// another_feature = []

fn main() {
    println!("Running application...");

    // Check if `my_feature` is enabled
    #[cfg(feature = "my_feature")]
    {
        println!("'my_feature' is enabled!");
        my_feature_function();
    }

    // Check if `another_feature` is enabled
    #[cfg(feature = "another_feature")]
    {
        println!("'another_feature' is enabled!");
        another_feature_function();
    }

    // This code always runs, regardless of feature flags
    println!("Common application logic.");
}

// This function only gets compiled if `my_feature` is enabled
#[cfg(feature = "my_feature")]
fn my_feature_function() {
    println!("Executing code specific to 'my_feature'.");
}

// This function only gets compiled if `another_feature` is enabled
#[cfg(feature = "another_feature")]
fn another_feature_function() {
    println!("Executing code specific to 'another_feature'.");
}

/*
To run with features:
cargo run --features my_feature
cargo run --features another_feature
cargo run --features "my_feature another_feature"
cargo run // Runs with no features enabled
*/