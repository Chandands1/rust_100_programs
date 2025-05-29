#[derive(Debug, Clone, Copy)]
struct Point {
    x: f64,
    y: f64,
}

impl Point {
    // Constructor method
    fn new(x: f64, y: f64) -> Self {
        Point { x, y }
    }

    // Calculate distance from origin
    fn distance_from_origin(&self) -> f64 {
        (self.x.powi(2) + self.y.powi(2)).sqrt()
    }

    // Calculate distance to another point
    fn distance_to(&self, other: &Point) -> f64 {
        ((self.x - other.x).powi(2) + (self.y - other.y).powi(2)).sqrt()
    }

    // Move point by given deltas
    fn translate(&mut self, dx: f64, dy: f64) {
        self.x += dx;
        self.y += dy;
    }

    // Create midpoint between two points
    fn midpoint(&self, other: &Point) -> Point {
        Point {
            x: (self.x + other.x) / 2.0,
            y: (self.y + other.y) / 2.0,
        }
    }
}

fn main() {
    let mut p1 = Point::new(3.0, 4.0);
    let p2 = Point::new(6.0, 8.0);

    println!("Point 1: {:?}", p1);
    println!("Point 2: {:?}", p2);

    println!("Distance from origin for p1: {:.2}", p1.distance_from_origin());
    println!("Distance between p1 and p2: {:.2}", p1.distance_to(&p2));

    p1.translate(1.0, -1.0);
    println!("After translation: {:?}", p1);

    let midpoint = p1.midpoint(&p2);
    println!("Midpoint: {:?}", midpoint);
}
