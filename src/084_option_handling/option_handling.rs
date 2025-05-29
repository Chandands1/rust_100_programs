fn main() {
    // ---
    // ## Safe Unwrap with `unwrap_or`
    // ---

    // An `Option` that contains a value
    let some_value: Option<i32> = Some(100);

    // Using `unwrap_or` to get the value or a default if `None`
    // This is a safe way to "unwrap" as it provides a fallback.
    let unwrapped_value = some_value.unwrap_or(0);
    println!("Unwrapped value (with unwrap_or): {}", unwrapped_value);

    // An `Option` that is `None`
    let no_value: Option<i32> = None;

    // Using `unwrap_or` on a `None` Option
    let unwrapped_none = no_value.unwrap_or(50);
    println!("Unwrapped none (with unwrap_or): {}", unwrapped_none);

    // ---
    // ## Safe Unwrap with `unwrap_or_else`
    // ---

    // `unwrap_or_else` takes a closure that is executed only if the Option is `None`.
    // This is useful for providing a default value that might be expensive to compute.
    let some_value_else: Option<String> = Some(String::from("Hello"));
    let unwrapped_value_else = some_value_else.unwrap_or_else(|| {
        // This closure will not be executed
        println!("Computing default for Some value...");
        String::from("Default String")
    });
    println!("Unwrapped value (with unwrap_or_else): {}", unwrapped_value_else);

    let no_value_else: Option<String> = None;
    let unwrapped_none_else = no_value_else.unwrap_or_else(|| {
        // This closure will be executed
        println!("Computing default for None value...");
        String::from("Another Default String")
    });
    println!("Unwrapped none (with unwrap_or_else): {}", unwrapped_none_else);

    // ---
    // ## Safe Unwrap with `map` and `unwrap_or`
    // ---

    // You can transform the value inside an `Option` and then safely unwrap.
    let number_option: Option<i32> = Some(20);
    let doubled_number = number_option
        .map(|n| n * 2) // Doubles the number if `Some`, otherwise propagates `None`
        .unwrap_or(0); // Provides a default if the original was `None`
    println!("Doubled number (with map and unwrap_or): {}", doubled_number);

    let no_number_option: Option<i32> = None;
    let default_number = no_number_option
        .map(|n| n * 2) // This map will not be executed as it's `None`
        .unwrap_or(10); // Provides the default
    println!("Default number (with map and unwrap_or): {}", default_number);
}