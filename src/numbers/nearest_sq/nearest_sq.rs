fn nearest_sq(n: u32) -> u32 {
    (n as f64).sqrt().round().powi(2) as u32
}

#[cfg(test)]
mod tests {
    use super::nearest_sq;

    #[test]
    fn sample_tests() {
        // assertion(expected, n)
        assertion(1, 1);
        assertion(1, 2);
        assertion(9, 10);
        assertion(121, 111);
        assertion(10000, 9999);
    }

    fn assertion(expected: u32, n: u32) {
        let actual = nearest_sq(n);
        assert!(
            expected == actual,
            "\nTest failed!\n expected: {}\n actual: {}\n n: {}\n", expected, actual, n
        );
    }
}