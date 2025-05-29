use std::sync::{Arc, Mutex}; // Import Atomic Reference Count and Mutex
use std::thread; // Import the thread module

fn main() {
    // Create an Arc (Atomic Reference Count) to share ownership of the Mutex across threads.
    // The Mutex protects the counter, allowing only one thread to access it at a time.
    let counter = Arc::new(Mutex::new(0));

    let mut handles = vec![]; // A vector to store the handles of spawned threads

    // Spawn 10 threads
    for _ in 0..10 {
        // Clone the Arc for each new thread.
        // This increments the reference count, indicating another owner of the data.
        let counter_clone = Arc::clone(&counter);

        let handle = thread::spawn(move || {
            // Lock the Mutex to get exclusive access to the counter.
            // `lock()` returns a `Result`, so `unwrap()` is used to get the `MutexGuard`.
            // The `MutexGuard` automatically unlocks the mutex when it goes out of scope.
            let mut num = counter_clone.lock().unwrap();

            // Increment the counter
            *num += 1;
        });
        handles.push(handle); // Add the thread handle to our vector
    }

    // Wait for all threads to complete their execution
    for handle in handles {
        handle.join().unwrap(); // Join each thread, panicking if a thread panics
    }

    // After all threads have finished, lock the Mutex one last time to read the final value.
    let final_count = counter.lock().unwrap();
    println!("Final counter value: {}", *final_count);
}