// A generic function to swap two values of any type `T`.
// The `<T>` syntax declares a generic type parameter named `T`.
// The function takes two mutable references to values of type `T`.
fn swap<T>(a: &mut T, b: &mut T) {
    // std::mem::swap provides a safe and efficient way to swap two values.
    std::mem::swap(a, b);
}

fn main() {
    // --- Swap two integers ---
    let mut x = 1;
    let mut y = 2;
    println!("Before swap: x = {}, y = {}", x, y);
    
    // Call the generic swap function with i32 types.
    swap(&mut x, &mut y);
    println!("After swap:  x = {}, y = {}", x, y);

    println!("---");

    // --- Swap two characters ---
    let mut char1 = 'A';
    let mut char2 = 'B';
    println!("Before swap: char1 = {}, char2 = {}", char1, char2);

    // Call the same generic swap function with char types.
    swap(&mut char1, &mut char2);
    println!("After swap:  char1 = {}, char2 = {}", char1, char2);
}