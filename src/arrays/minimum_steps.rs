fn minimum_steps(nums: &[i32], value: i32) -> usize {
    let mut arr = nums.to_vec();
    arr.sort();
    let  mut sum = 0i32;
    for i in 0.. arr.len() {
        sum += arr[i];
        if sum >= value {
            return  i;
        }
    }
    0usize
}

//https://www.codewars.com/kata/5a91a7c5fd8c061367000002/train/rust


// https://doc.rust-lang.org/stable/rust-by-example/testing/unit_testing.html

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn basic_tests() {
        assert_eq!(minimum_steps(&[4, 6, 3], 7), 1);
        assert_eq!(minimum_steps(&[10, 9, 9, 8], 17), 1);
        assert_eq!(minimum_steps(&[8, 9, 10, 4, 2], 23), 3);
        assert_eq!(minimum_steps(&[19, 98, 69, 28, 75, 45, 17, 98, 67], 464), 8);
        assert_eq!(minimum_steps(&[4, 6, 3], 2), 0);
    }
}

