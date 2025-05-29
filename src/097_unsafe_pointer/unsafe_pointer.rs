fn main() {
    let mut value = 10;
    let new_value = 20;

    // Create an immutable raw pointer to `value`
    let r1: *const i32 = &value;

    // Create a mutable raw pointer to `value`
    let r2: *mut i32 = &mut value;

    // Use an `unsafe` block to dereference raw pointers.
    // This tells the Rust compiler that we are taking responsibility for memory safety.
    unsafe {
        println!("Value pointed to by r1 (immutable): {}", *r1);
        println!("Value pointed to by r2 (mutable): {}", *r2);

        // Dereference and modify the value through the mutable raw pointer
        *r2 = new_value;
        println!("Value after modifying via r2: {}", *r2);
        println!("Original `value` variable: {}", value);
    }

    // ---
    // ## Creating a raw pointer from an arbitrary address (very unsafe)
    // ---
    // This is generally not something you'd do in safe Rust code.
    // It's primarily for interacting with C FFI or operating system memory.
    let address = 0x12345678 as *mut u8; // Example: a hypothetical memory address

    // Dereferencing an arbitrary address without knowing what's there
    // is highly dangerous and can lead to segfaults or corrupted memory.
    unsafe {
        // This line *will likely crash* your program unless 0x12345678
        // is a valid, accessible, and meaningful address in your process.
        // println!("Value at arbitrary address: {}", *address);
        // Uncomment the line above only if you know what you are doing and
        // are prepared for a crash.
    }

    println!("\nRaw pointer operations complete. Be cautious with `unsafe`!");
}