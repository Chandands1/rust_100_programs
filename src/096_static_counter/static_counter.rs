// Declare a mutable static variable.
// `static mut` requires an `unsafe` block for access due to potential data races.
// It's generally discouraged in favor of `static` with `Mutex` or atomic types (`std::sync::atomic`).
static mut COUNTER: u32 = 0;

fn main() {
    println!("Initial COUNTER value: {}", unsafe { COUNTER });

    // Accessing and modifying `static mut` variables requires an `unsafe` block.
    // This is because multiple threads could access and modify it simultaneously
    // without synchronization, leading to undefined behavior.
    unsafe {
        COUNTER += 1;
        println!("COUNTER after increment 1: {}", COUNTER);

        COUNTER += 5;
        println!("COUNTER after increment 2: {}", COUNTER);
    }

    // Even reading requires `unsafe`
    println!("Final COUNTER value (unsafe read): {}", unsafe { COUNTER });

    // Note: For concurrent access in multi-threaded scenarios,
    // `static` with `std::sync::Mutex` or `std::sync::atomic` types are preferred
    // over `static mut` to ensure thread safety.
}