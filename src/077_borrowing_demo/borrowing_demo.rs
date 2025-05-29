fn main() {
    let s1 = String::from("hello");

    // Immutable borrow
    let len = calculate_length(&s1);
    println!("Length of '{}': {}", s1, len);  // s1 still valid

    // Mutable borrow
    let mut s2 = String::from("world");
    modify_string(&mut s2);
    println!("Modified string: {}", s2);

    // Multiple immutable borrows allowed
    let r1 = &s1;
    let r2 = &s1;
    println!("r1: {}, r2: {}", r1, r2);

    // Mutable borrow rules demo
    let mut s3 = String::from("rust");
    let r3 = &mut s3;
    // let r4 = &mut s3;  // Error: can't have two mutable references
    // let r5 = &s3;      // Error: can't mix mutable and immutable
    println!("r3: {}", r3);

    // Scope-based borrowing
    {
        let r6 = &mut s3;
        r6.push_str(" is awesome");
    }  // r6 goes out of scope here
    let r7 = &s3;  // Now allowed
    println!("r7: {}", r7);

    // Dangling reference prevention (won't compile)
    // let reference_to_nothing = dangle();
}

fn calculate_length(s: &String) -> usize {
    s.len()
}  // s goes out of scope but doesn't drop anything

fn modify_string(s: &mut String) {
    s.push_str("!");
}  // mutable borrow ends here

// This function won't compile - demonstrates dangling reference prevention
/*
fn dangle() -> &String {
    let s = String::from("dangling");
    &s  // Error: returns reference to local variable
}
*/