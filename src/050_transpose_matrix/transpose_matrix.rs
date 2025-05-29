fn transpose_matrix(matrix: &Vec<Vec<i32>>) -> Vec<Vec<i32>> {
    let rows = matrix.len();
    let cols = matrix[0].len();
    
    let mut transposed = vec![vec![0; rows]; cols];
    
    for i in 0..rows {
        for j in 0..cols {
            transposed[j][i] = matrix[i][j];
        }
    }
    
    transposed
}

fn main() {
    let matrix = vec![
        vec![1, 2, 3],
        vec![4, 5, 6],
        vec![7, 8, 9]
    ];
    
    println!("Original Matrix:");
    for row in &matrix {
        println!("{:?}", row);
    }
    
    let transposed = transpose_matrix(&matrix);
    
    println!("\nTransposed Matrix:");
    for row in &transposed {
        println!("{:?}", row);
    }
}