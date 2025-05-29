fn matrix_multiply(a: &Vec<Vec<i32>>, b: &Vec<Vec<i32>>) -> Vec<Vec<i32>> {
    assert_eq!(a[0].len(), b.len(), "Matrix dimensions incompatible for multiplication");
    
    let m = a.len();
    let n = b[0].len();
    let p = b.len();
    
    let mut result = vec![vec![0; n]; m];
    
    for i in 0..m {
        for j in 0..n {
            for k in 0..p {
                result[i][j] += a[i][k] * b[k][j];
            }
        }
    }
    
    result
}

fn main() {
    let matrix_a = vec![
        vec![1, 2, 3],
        vec![4, 5, 6]
    ];
    
    let matrix_b = vec![
        vec![7, 8],
        vec![9, 10],
        vec![11, 12]
    ];
    
    let product = matrix_multiply(&matrix_a, &matrix_b);
    
    println!("Matrix A:");
    for row in &matrix_a {
        println!("{:?}", row);
    }
    
    println!("\nMatrix B:");
    for row in &matrix_b {
        println!("{:?}", row);
    }
    
    println!("\nProduct:");
    for row in &product {
        println!("{:?}", row);
    }
}