fn print_pyramid(height: usize) {
    for i in 0..height {
        // Print leading spaces
        for _ in 0..(height - i - 1) {
            print!(" ");
        }
        
        // Print stars
        for _ in 0..(2 * i + 1) {
            print!("*");
        }
        
        println!();
    }
}

fn main() {
    let height = 5;
    println!("Pyramid Pattern (Height: {}):", height);
    print_pyramid(height);
}