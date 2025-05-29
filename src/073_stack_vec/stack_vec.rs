#[derive(Debug)]
pub struct Stack<T> {
    elements: Vec<T>,
}

impl<T> Stack<T> {
    /// Create a new empty stack
    pub fn new() -> Self {
        Stack { elements: Vec::new() }
    }

    /// Push an item onto the stack
    pub fn push(&mut self, item: T) {
        self.elements.push(item);
    }

    /// Pop an item from the stack (returns Option<T>)
    pub fn pop(&mut self) -> Option<T> {
        self.elements.pop()
    }

    /// Peek at the top item without removing it
    pub fn peek(&self) -> Option<&T> {
        self.elements.last()
    }

    /// Check if stack is empty
    pub fn is_empty(&self) -> bool {
        self.elements.is_empty()
    }

    /// Get current stack size
    pub fn size(&self) -> usize {
        self.elements.len()
    }
}

fn main() {
    let mut stack = Stack::new();

    stack.push(10);
    stack.push(20);
    stack.push(30);

    println!("Stack: {:?}", stack);
    println!("Size: {}", stack.size());
    println!("Top item: {:?}", stack.peek());

    while let Some(item) = stack.pop() {
        println!("Popped: {}", item);
    }

    println!("Is empty? {}", stack.is_empty());
}