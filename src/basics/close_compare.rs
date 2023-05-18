fn close_compare(a: f64, b: f64, margin: f64) -> i8 {
    if (a - b).abs() <= margin {
       return  0;
    }
    if (a - b) <  margin {
        return  -1;
    }
    1
}

// Add your tests here.
// See https://doc.rust-lang.org/stable/rust-by-example/testing/unit_testing.html
// https://www.codewars.com/kata/56453a12fcee9a6c4700009c/train/rust
#[cfg(test)]
mod tests {
    use super::close_compare;

    const ERR_MSG: &str = "\nYour result (left) did not match the expected output (right)";

    fn dotest(a: f64, b: f64, margin: f64, expected: i8) {
        assert_eq!(close_compare(a, b, margin), expected, "{ERR_MSG} with a = {a}, b = {b}, margin = {margin}")
    }

    #[test]
    fn fixed_tests() {
        dotest(4.0, 5.0, 0.0, -1);
        dotest(5.0, 5.0, 0.0, 0);
          dotest(6.0, 5.0, 0.0, 1);
         dotest(2.0, 5.0, 3.0, 0);
         dotest(5.0, 5.0, 3.0, 0);
         dotest(8.0, 5.0, 3.0, 0);
         dotest(8.1, 5.0, 3.0, 1);
        dotest(1.99, 5.0, 3.0, -1);
    }
}

