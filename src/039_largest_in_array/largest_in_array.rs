fn find_largest(arr: &[i32]) -> i32 {
    let mut largest = arr[0];
    for &num in arr {
        if num > largest {
            largest = num;
        }
    }
    largest
}

fn main() {
    let numbers = [10, 5, 25, 50, 15];
    println!("Largest number: {}", find_largest(&numbers));
}