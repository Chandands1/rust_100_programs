fn insertion_sort(arr: &mut [i32]) {
    for i in 1..arr.len() {
        let current = arr[i];
        let mut j = i;
        
        while j > 0 && arr[j - 1] > current {
            arr[j] = arr[j - 1];
            j -= 1;
        }
        
        arr[j] = current;
    }
}

fn main() {
    let mut numbers = [12, 11, 13, 5, 6];
    println!("Unsorted: {:?}", numbers);
    
    insertion_sort(&mut numbers);
    println!("Sorted: {:?}", numbers);
}