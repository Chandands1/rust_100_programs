// Define a struct to hold the iterator's state.
// It will count from 1 up to a maximum value.
struct Counter {
    count: u32, // The current number in the sequence
    max: u32,   // The number where the iterator will stop
}

// Implement a `new` function to create a Counter instance.
impl Counter {
    fn new(max: u32) -> Counter {
        Counter { count: 0, max }
    }
}

// Implement the `Iterator` trait for our `Counter` struct.
impl Iterator for Counter {
    // We need to define the type of item our iterator will return.
    type Item = u32;

    // The `next` method is the core of the iterator.
    // It's called for each step of the iteration.
    fn next(&mut self) -> Option<Self::Item> {
        // Increment our count.
        self.count += 1;

        // Check if we've finished counting.
        if self.count <= self.max {
            // If not, return the current count wrapped in `Some`.
            Some(self.count)
        } else {
            // If we have finished, return `None` to signal the end.
            None
        }
    }
}

fn main() {
    // Create a new Counter that will count up to 5.
    let mut counter = Counter::new(5);

    // The `Iterator` trait allows us to use our `Counter` in a for loop.
    println!("Using the custom iterator in a for loop:");
    for i in counter {
        println!("i = {}", i);
    }
    
    // We can also call `next` manually.
    let mut counter_manual = Counter::new(3);
    println!("\nCalling next() manually:");
    println!("{:?}", counter_manual.next()); // Some(1)
    println!("{:?}", counter_manual.next()); // Some(2)
    println!("{:?}", counter_manual.next()); // Some(3)
    println!("{:?}", counter_manual.next()); // None
}