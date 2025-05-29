fn matrix_addition(a: &Vec<Vec<i32>>, b: &Vec<Vec<i32>>) -> Vec<Vec<i32>> {
    assert_eq!(a.len(), b.len(), "Matrices must have same dimensions");
    let mut result = vec![vec![0; a[0].len()]; a.len()];
    
    for i in 0..a.len() {
        assert_eq!(a[i].len(), b[i].len(), "Matrices must have same dimensions");
        for j in 0..a[i].len() {
            result[i][j] = a[i][j] + b[i][j];
        }
    }
    
    result
}

fn main() {
    let matrix1 = vec![
        vec![1, 2, 3],
        vec![4, 5, 6],
        vec![7, 8, 9]
    ];
    
    let matrix2 = vec![
        vec![9, 8, 7],
        vec![6, 5, 4],
        vec![3, 2, 1]
    ];
    
    let sum = matrix_addition(&matrix1, &matrix2);
    
    println!("Matrix 1:");
    for row in &matrix1 {
        println!("{:?}", row);
    }
    
    println!("\nMatrix 2:");
    for row in &matrix2 {
        println!("{:?}", row);
    }
    
    println!("\nSum:");
    for row in &sum {
        println!("{:?}", row);
    }
}