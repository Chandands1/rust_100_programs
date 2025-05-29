fn print_right_triangle(n: usize) {
    for i in 1..=n {
        for _ in 1..=i {
            print!("* ");
        }
        println!();
    }
}

fn main() {
    let size = 5;
    println!("Right Triangle Pattern (Size: {}):", size);
    print_right_triangle(size);
}