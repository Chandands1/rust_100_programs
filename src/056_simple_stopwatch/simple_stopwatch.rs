use std::time::Instant;
use std::io::{self, Write};
use std::thread;
use std::time::Duration;

fn stopwatch() {
    println!("Press Enter to start the stopwatch");
    io::stdin().read_line(&mut String::new()).unwrap();
    
    let start = Instant::now();
    println!("Stopwatch started! Press Enter to stop...");
    
    // Spawn a thread to display elapsed time
    let running = std::sync::Arc::new(std::sync::atomic::AtomicBool::new(true));
    let running_clone = running.clone();
    
    thread::spawn(move || {
        while running_clone.load(std::sync::atomic::Ordering::Relaxed) {
            print!("\rElapsed: {:.2}s", start.elapsed().as_secs_f64());
            io::stdout().flush().unwrap();
            thread::sleep(Duration::from_millis(50));
        }
    });
    
    // Wait for user to stop
    io::stdin().read_line(&mut String::new()).unwrap();
    running.store(false, std::sync::atomic::Ordering::Relaxed);
    
    let duration = start.elapsed();
    println!("\nFinal time: {:.2} seconds", duration.as_secs_f64());
}

fn main() {
    println!("Simple Stopwatch");
    println!("----------------");
    stopwatch();
}