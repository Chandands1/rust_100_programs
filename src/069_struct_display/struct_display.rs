use std::fmt;

#[derive(Debug)]
struct Point {
    x: f64,
    y: f64,
}

impl Point {
    fn new(x: f64, y: f64) -> Self {
        Point { x, y }
    }
}

impl fmt::Display for Point {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "({}, {})", self.x, self.y)
    }
}

fn main() {
    let point = Point::new(3.5, 4.2);
    
    // Uses Display implementation
    println!("Point coordinates: {}", point);
    
    // Uses Debug implementation
    println!("Debug view: {:?}", point);
    
    // Can be used in format strings
    let s = format!("Formatted point: {}", point);
    println!("{}", s);
}