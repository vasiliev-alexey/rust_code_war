fn minimum(arr: &[i32]) -> i32 {
    *arr.iter().min().unwrap()
}

fn maximum(arr: &[i32]) -> i32 {
    *arr.iter().max().unwrap()
}

// Add your tests here.
// See https://doc.rust-lang.org/stable/rust-by-example/testing/unit_testing.html

#[cfg(test)]
mod tests {
    use super::{minimum, maximum};

    const ERR_MSG: &str = "\nYour result (left) did not match the expected output (right)";

    fn dotest(arr: &[i32], expected_min: i32, expected_max: i32) {
        assert_eq!(minimum(arr), expected_min, "{ERR_MSG} with function minimum and arr = {arr:?}");
        assert_eq!(maximum(arr), expected_max, "{ERR_MSG} with function maximum and arr = {arr:?}");
    }

    #[test]
    fn fixed_tests() {
        dotest(&[-52, 56, 30, 29, -54, 0, -110], -110, 56);
        dotest(&[42, 54, 65, 87, 0], 0, 87);
        dotest(&[1, 2, 3, 4, 5, 10], 1, 10);
        dotest(&[-1, -2, -3, -4, -5, -10, 534, 43, 2, 1, 3, 4, 5, 5, 443, 443, 555, 555], -10, 555);
        dotest(&[9], 9, 9);
        dotest(&[4,6,2,1,9,63,-134,566], -134, 566);
    }
}

/*
Your task is to make two functions ( max and min, or maximum and minimum, etc., depending on the language ) that receive a list of integers as input, and return the largest and lowest number in that list, respectively.
Examples (Input -> Output)
* [4,6,2,1,9,63,-134,566]         -> max = 566, min = -134
* [-52, 56, 30, 29, -54, 0, -110] -> min = -110, max = 56
* [42, 54, 65, 87, 0]             -> min = 0, max = 87
* [5]                             -> min = 5, max = 5
  Notes
  You may consider that there will not be any empty arrays/vectors.
*/