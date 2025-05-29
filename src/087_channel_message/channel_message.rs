use std::sync::mpsc; // Import the multiple producer, single consumer (mpsc) channel module
use std::thread;      // Import the thread module
use std::time::Duration; // Import Duration for sleeping

fn main() {
    // Create a new channel. `mpsc::channel()` returns a tuple: (sender, receiver).
    let (tx, rx) = mpsc::channel();

    // Spawn a producer thread
    let producer_handle = thread::spawn(move || {
        let messages = vec![
            String::from("Hi from thread 1"),
            String::from("Hello from thread 1"),
            String::from("Greetings from thread 1"),
        ];

        for msg in messages {
            println!("Producer sending: {}", msg);
            // Send the message through the channel. `tx.send()` returns a `Result`.
            tx.send(msg).unwrap();
            thread::sleep(Duration::from_millis(500)); // Simulate some work
        }
    });

    // Spawn another producer thread
    let tx2 = mpsc::Sender::clone(&tx); // Clone the sender to have multiple producers
    let producer_handle2 = thread::spawn(move || {
        let messages = vec![
            String::from("Message from thread 2, part 1"),
            String::from("Message from thread 2, part 2"),
        ];

        for msg in messages {
            println!("Producer 2 sending: {}", msg);
            tx2.send(msg).unwrap();
            thread::sleep(Duration::from_millis(700)); // Simulate some work
        }
    });

    // The main thread acts as the consumer
    println!("Consumer ready to receive messages...");

    // Receive messages from the channel.
    // `rx.iter()` creates an iterator that will block until a message is received
    // and will continue to yield messages until the sending half of the channel is dropped.
    for received_msg in rx.iter() {
        println!("Consumer received: {}", received_msg);
    }

    // Wait for producer threads to finish
    producer_handle.join().unwrap();
    producer_handle2.join().unwrap();

    println!("All messages processed and threads finished.");
}