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
        assert_eq!(expected, actual, "\nTest failed!\n expected: {}\n actual: {}\n n: {}\n", expected, actual, n);
    }
}

/*
Your task is to find the nearest square number, nearest_sq(n) or nearestSq(n), of a positive integer n.
For example, if n = 111, then nearest\_sq(n) (nearestSq(n)) equals 121, since 111 is closer to 121, the square of 11, than 100, the square of 10.
If the n is already the perfect square (e.g. n = 144, n = 81, etc.), you need to just return n.
Good luck :)
*/