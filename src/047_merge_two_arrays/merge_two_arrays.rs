fn merge_arrays(arr1: &[i32], arr2: &[i32]) -> Vec<i32> {
    let mut merged = Vec::with_capacity(arr1.len() + arr2.len());
    merged.extend(arr1);
    merged.extend(arr2);
    merged
}

fn main() {
    let arr1 = [1, 3, 5];
    let arr2 = [2, 4, 6];
    
    let merged = merge_arrays(&arr1, &arr2);
    println!("Merged array: {:?}", merged);
}


//For a sorted merge (assuming input arrays are sorted):

fn merge_sorted_arrays(arr1: &[i32], arr2: &[i32]) -> Vec<i32> {
    let mut result = Vec::with_capacity(arr1.len() + arr2.len());
    let (mut i, mut j) = (0, 0);
    
    while i < arr1.len() && j < arr2.len() {
        if arr1[i] < arr2[j] {
            result.push(arr1[i]);
            i += 1;
        } else {
            result.push(arr2[j]);
            j += 1;
        }
    }
    
    result.extend(&arr1[i..]);
    result.extend(&arr2[j..]);
    result
}

fn main() {
    let arr1 = [1, 3, 5];
    let arr2 = [2, 4, 6];
    
    let merged = merge_sorted_arrays(&arr1, &arr2);
    println!("Merged sorted array: {:?}", merged);
}