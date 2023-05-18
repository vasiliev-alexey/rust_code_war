fn determinant(matrix: &[Vec<i64>]) -> i64 {
    if matrix.len() == 1 {
        return *matrix.get(0).unwrap().get(0).unwrap();
    }

    if matrix.len() == 2 {
        return matrix.get(0).unwrap().get(0).unwrap()
            * matrix.get(1).unwrap().get(1).unwrap()
            -
            matrix.get(0).unwrap().get(1).unwrap()
                * matrix.get(1).unwrap().get(0).unwrap();
    }

    let   row_resolve = matrix.get(0).unwrap();
    let mut rez = 0 as i64;


    for i in 0..row_resolve.len() {
        let calc_arr =
            matrix.iter().enumerate().filter(|(x, _)| *x as u8 != 0).map(|(_, y)| y)
                .collect::<Vec<_>>();
        let r = calc_arr.iter().map(|x| x.iter().enumerate()
            .filter(|(ind, _)| *ind != i).map(|x| *x.1).collect::<Vec<i64>>()
        ).collect::<Vec<_>>();

        //let rr = determinant(&r);

        let sign = if i % 2 == 0 { 1 } else { -1 };

        rez = rez +  determinant(&r) * sign * row_resolve.get(i).unwrap();
    }

    rez
}


#[cfg(test)]
mod tests {
    use super::determinant;

    const ERR_MSG: &str = "\nYour result (left) did not match the expected output (right)";

    fn dotest(a: &[Vec<i64>], expected: i64) {
        assert_eq!(determinant(a), expected, "{ERR_MSG}")
    }

    #[test]
    fn sample_tests() {
        dotest(&[vec![1]], 1);
         dotest(&[vec![1, 3], vec![2, 5]], -1);
        dotest(&[vec![2, 5, 3], vec![1, -2, -1], vec![1, 3, 4]], -20);
        dotest(&[vec![1, 2, 3, 4], vec! [5, 6, 7, 8], vec!  [9, 1, 2, 3] , vec![4, 5, 6, 9]], -72);
    }
}
