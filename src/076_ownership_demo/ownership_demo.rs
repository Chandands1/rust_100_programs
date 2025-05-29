fn main() {
    // String type (heap-allocated, owns its data)
    let s1 = String::from("hello");
    println!("s1: {}", s1);  // Works fine
    
    // Move s1 to s2 (ownership transfer)
    let s2 = s1;
    // println!("s1: {}", s1);  // Error! s1 is no longer valid
    println!("s2: {}", s2);  // Works - s2 now owns the data

    // Clone creates a deep copy (new ownership)
    let s3 = s2.clone();
    println!("s2: {}, s3: {}", s2, s3);  // Both valid

    // Integer type (implements Copy trait)
    let x = 5;
    let y = x;  // Copy happens automatically
    println!("x: {}, y: {}", x, y);  // Both valid

    // Ownership in functions
    takes_ownership(s3);
    // println!("s3: {}", s3);  // Error! s3 was moved into the function

    let s4 = gives_ownership();
    println!("s4: {}", s4);

    let (s5, len) = calculate_length(s4);
    println!("'{}' has length {}", s5, len);
}

fn takes_ownership(some_string: String) {
    println!("takes_ownership: {}", some_string);
}  // some_string goes out of scope and is dropped

fn gives_ownership() -> String {
    let some_string = String::from("world");
    some_string  // Ownership is moved out
}

fn calculate_length(s: String) -> (String, usize) {
    let length = s.len();
    (s, length)  // Return both the string and its length
}