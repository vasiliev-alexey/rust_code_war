fn slice_plus_slice(xs: &[i32], ys: &[i32]) -> i32 {

    xs.iter().sum::<i32>() + ys.iter().sum::<i32>()
}


//https://www.codewars.com/kata/5a2be17aee1aaefe2a000151/train/rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sample_tests() {
        assert_eq!(slice_plus_slice(&vec![1], &vec![4]), 5);
        assert_eq!(slice_plus_slice(&vec![1, 2, 3], &vec![4, 5, 6]), 21);
        assert_eq!(slice_plus_slice(&vec![-1, -2, -3], &vec![-4, -5, -6]), -21);
    }
}