fn gcd(a: u32, b: u32) -> u32 {
    if a > 0 {
        return gcd(b % a, a);
    }
    b
}

// Add your tests here.
// See https://doc.rust-lang.org/stable/rust-by-example/testing/unit_testing.html

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sample_tests() {
        assert_eq!(gcd(30, 12), 6);
        assert_eq!(gcd(8, 9), 1);
        assert_eq!(gcd(1, 1), 1);
    }
}

/* Greatest common divisor
Find the greatest common divisor of two positive integers. The integers can be large, so you need to find a clever solution.

The inputs x and y are always greater or equal to 1, so the greatest common divisor will always be an integer that is also greater or equal to 1.
*/