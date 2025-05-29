use std::thread; // Import the thread module

fn main() {
    // Create a new thread
    let handle = thread::spawn(|| {
        // This closure is the code that the new thread will execute
        println!("Hello from a new thread!");
    });

    // Code in the main thread continues to execute concurrently
    println!("Hello from the main thread!");

    // Wait for the new thread to finish its execution
    // `join()` returns a `Result`, so we use `unwrap()` to get the inner value.
    // If the thread panics, `unwrap()` will also panic.
    handle.join().unwrap();

    println!("Main thread finished after waiting for the new thread.");
}