fn linear_search(arr: &[i32], target: i32) -> Option<usize> {
    for (index, &num) in arr.iter().enumerate() {
        if num == target {
            return Some(index);
        }
    }
    None
}

fn main() {
    let numbers = [10, 25, 30, 42, 55];
    let target = 30;

    match linear_search(&numbers, target) {
        Some(index) => println!("{} found at index {}", target, index),
        None => println!("{} not found in the array", target),
    }
}