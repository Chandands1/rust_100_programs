use std::collections::VecDeque;

#[derive(Debug)]
pub struct Queue<T> {
    elements: VecDeque<T>,
}

impl<T> Queue<T> {
    /// Create a new empty queue
    pub fn new() -> Self {
        Queue {
            elements: VecDeque::new(),
        }
    }

    /// Add an item to the back of the queue (enqueue)
    pub fn enqueue(&mut self, item: T) {
        self.elements.push_back(item);
    }

    /// Remove an item from the front of the queue (dequeue)
    pub fn dequeue(&mut self) -> Option<T> {
        self.elements.pop_front()
    }

    /// Peek at the front item without removing it
    pub fn peek_front(&self) -> Option<&T> {
        self.elements.front()
    }

    /// Peek at the back item without removing it
    pub fn peek_back(&self) -> Option<&T> {
        self.elements.back()
    }

    /// Check if queue is empty
    pub fn is_empty(&self) -> bool {
        self.elements.is_empty()
    }

    /// Get current queue size
    pub fn size(&self) -> usize {
        self.elements.len()
    }
}

fn main() {
    let mut queue = Queue::new();

    queue.enqueue("Customer 1");
    queue.enqueue("Customer 2");
    queue.enqueue("Customer 3");

    println!("Queue: {:?}", queue);
    println!("Size: {}", queue.size());
    println!("Front: {:?}", queue.peek_front());
    println!("Back: {:?}", queue.peek_back());

    while let Some(customer) = queue.dequeue() {
        println!("Processing: {}", customer);
    }

    println!("Is empty? {}", queue.is_empty());
}