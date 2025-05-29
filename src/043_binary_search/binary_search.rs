fn binary_search(arr: &[i32], target: i32) -> Option<usize> {
    let mut low = 0;
    let mut high = arr.len() - 1;

    while low <= high {
        let mid = (low + high) / 2;
        match arr[mid].cmp(&target) {
            std::cmp::Ordering::Equal => return Some(mid),
            std::cmp::Ordering::Less => low = mid + 1,
            std::cmp::Ordering::Greater => high = mid - 1,
        }
    }
    None
}

fn main() {
    let sorted_numbers = [10, 20, 30, 40, 50, 60, 70];
    let target = 40;

    match binary_search(&sorted_numbers, target) {
        Some(index) => println!("{} found at index {}", target, index),
        None => println!("{} not found in the array", target),
    }
}