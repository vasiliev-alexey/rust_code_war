fn invert(values: &[i32]) -> Vec<i32> {
    values.iter().map(|x| -x).collect()
}
// Add your tests here.
// See https://doc.rust-lang.org/stable/rust-by-example/testing/unit_testing.html


#[cfg(test)]
mod tests {
    use super::invert;

    #[test]
    fn basic_tests() {
        assert_eq!(invert(&vec![1, 2, 3, 4, 5]), vec![-1, -2, -3, -4, -5]);
        assert_eq!(invert(&vec![1, -2, 3, -4, 5]), vec![-1, 2, -3, 4, -5]);
        assert_eq!(invert(&vec![]), vec![]);
    }
}