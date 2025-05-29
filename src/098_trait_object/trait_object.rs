// Define a trait (interface)
trait Printable {
    fn print_info(&self); // A method that implementors must provide
}

// Implement the `Printable` trait for a struct `Book`
struct Book {
    title: String,
    author: String,
}

impl Printable for Book {
    fn print_info(&self) {
        println!("Book: \"{}\" by {}", self.title, self.author);
    }
}

// Implement the `Printable` trait for a struct `Movie`
struct Movie {
    title: String,
    director: String,
    year: u16,
}

impl Printable for Movie {
    fn print_info(&self) {
        println!("Movie: \"{}\" ({}) directed by {}", self.title, self.year, self.director);
    }
}

fn main() {
    let book1 = Book {
        title: String::from("The Rust Programming Language"),
        author: String::from("Steve Klabnik and Carol Nichols"),
    };

    let movie1 = Movie {
        title: String::from("Inception"),
        director: String::from("Christopher Nolan"),
        year: 2010,
    };

    // Create a vector that can hold *any* type that implements the `Printable` trait.
    // This is a "trait object" and enables dynamic dispatch.
    // `Box<dyn Printable>` means a boxed (heap-allocated) type that implements `Printable`.
    let mut printable_items: Vec<Box<dyn Printable>> = Vec::new();

    // Add instances of `Book` and `Movie` to the vector.
    // They are boxed because trait objects are "fat pointers" and need a known size at compile time.
    printable_items.push(Box::new(book1));
    printable_items.push(Box::new(movie1));

    // Iterate through the collection and call the `print_info` method.
    // The exact implementation of `print_info` is determined at runtime (dynamic dispatch)
    // based on the concrete type stored in the Box.
    for item in printable_items {
        item.print_info();
    }
}