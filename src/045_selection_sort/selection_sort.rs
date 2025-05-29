fn selection_sort(arr: &mut [i32]) {
    let len = arr.len();
    for i in 0..len {
        let mut min_index = i;
        for j in i+1..len {
            if arr[j] < arr[min_index] {
                min_index = j;
            }
        }
        arr.swap(i, min_index);
    }
}

fn main() {
    let mut numbers = [64, 25, 12, 22, 11];
    println!("Unsorted: {:?}", numbers);
    
    selection_sort(&mut numbers);
    println!("Sorted: {:?}", numbers);
}