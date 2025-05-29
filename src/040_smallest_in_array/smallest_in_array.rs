fn find_smallest(arr: &[i32]) -> i32 {
    let mut smallest = arr[0];
    for &num in arr {
        if num < smallest {
            smallest = num;
        }
    }
    smallest
}

fn main() {
    let numbers = [10, 5, 25, 50, 15];
    println!("Smallest number: {}", find_smallest(&numbers));
}