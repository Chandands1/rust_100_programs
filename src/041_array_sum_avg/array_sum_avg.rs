fn array_sum_and_average(arr: &[i32]) -> (i32, f64) {
    let sum: i32 = arr.iter().sum();
    let average = sum as f64 / arr.len() as f64;
    (sum, average)
}

fn main() {
    let numbers = [10, 20, 30, 40, 50];
    let (sum, average) = array_sum_and_average(&numbers);
    
    println!("Array: {:?}", numbers);
    println!("Sum: {}", sum);
    println!("Average: {:.2}", average);
}