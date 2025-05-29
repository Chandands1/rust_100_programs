// A function that takes two string slices and returns the longest one.
// The lifetime 'a annotates that the returned reference is tied to the
// lifetime of the shortest-lived of the two input references.
fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    // Compare the lengths of the two string slices.
    if x.len() > y.len() {
        // Return the first string slice if it's longer.
        x
    } else {
        // Otherwise, return the second string slice.
        y
    }
}

fn main() {
    // Create two String objects.
    let string1 = String::from("abcd");
    let string2 = "xyz";

    // Call the `longest` function with slices of the two strings.
    // The result will be a reference to the longest of the two.
    let result = longest(string1.as_str(), string2);
    
    // Print the longest string.
    println!("The longest string is {}", result);

    // Another example with different string literals.
    let string3 = "a long string is long";
    {
        let string4 = String::from("short");
        // The result's lifetime is tied to string4 here.
        let result2 = longest(string3, &string4);
        println!("The longest string is {}", result2);
    } // string4 goes out of scope here.
}