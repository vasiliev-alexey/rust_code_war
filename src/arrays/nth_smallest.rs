fn nth_smallest(nums: &[i32], pos: usize) -> i32 {
    println!("{nums:?}");
    let mut   vec =  nums.to_vec();
    vec.sort();
    vec.into_iter().nth(pos-1).unwrap()
}

// https://doc.rust-lang.org/stable/rust-by-example/testing/unit_testing.html

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sample_tests() {
        assert_eq!(nth_smallest(&[3, 1, 2], 2), 2);
         assert_eq!(nth_smallest(&[15, 20, 7, 10, 4, 3], 3), 7);
         assert_eq!(nth_smallest(&[-5, -1, -6, -18], 4), -1);
         assert_eq!(nth_smallest(&[-102, -16, -1, -2, -367, -9], 5), -2);
         assert_eq!(nth_smallest(&[2, 169, 13, -5, 0, -1], 4), 2);
    }
}