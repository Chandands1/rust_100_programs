fn print_diamond(size: usize) {
    // Upper half of diamond
    for i in 0..size {
        for _ in 0..(size - i - 1) {
            print!(" ");
        }
        for _ in 0..(2 * i + 1) {
            print!("*");
        }
        println!();
    }
    
    // Lower half of diamond
    for i in (0..size-1).rev() {
        for _ in 0..(size - i - 1) {
            print!(" ");
        }
        for _ in 0..(2 * i + 1) {
            print!("*");
        }
        println!();
    }
}

fn main() {
    let size = 5;
    println!("Diamond Pattern (Size: {}):", size);
    print_diamond(size);
}