fn transpose(matrix: &[Vec<u8>]) -> Vec<Vec<u8>> {
    let row_cnt = matrix[0].len();
    let col_cnt = matrix.len();
    let mut res = Vec::with_capacity(row_cnt);
    for i in 0..row_cnt {
        let mut row = Vec::with_capacity(col_cnt);
        for j in 0..col_cnt {
            row.push(matrix[j][i]);
        }
        res.push(row);
    }
    res
}

//https://www.codewars.com/kata/52fba2a9adcd10b34300094c/train/rust
// Add your tests here.
// See https://doc.rust-lang.org/stable/rust-by-example/testing/unit_testing.html

#[cfg(test)]
mod tests {
    use super::transpose;

    #[test]
    fn sample_tests() {
        assert_eq!(transpose(&[vec![1]]), vec![vec![1]]);
        assert_eq!(transpose(&[vec![1, 2, 3]]), vec![vec![1], vec![2], vec![3]]);
        assert_eq!(transpose(&[vec![1, 2, 3], vec![4, 5, 6], vec![7, 8, 9]]), vec![vec![1, 4, 7], vec![2, 5, 8], vec![3, 6, 9]]);
        assert_eq!(transpose(&[vec![1, 0, 0], vec![0, 1, 0], vec![0, 0, 1], vec![0, 1, 0], vec![1, 0, 0]]), vec![vec![1, 0, 0, 0, 1], vec![0, 1, 0, 1, 0], vec![0, 0, 1, 0, 0]]);
    }
}
