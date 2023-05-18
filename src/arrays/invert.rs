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

/*
Given a set of numbers, return the additive inverse of each. Each positive becomes negatives, and the negatives become positives.

invert([1,2,3,4,5]) == [-1,-2,-3,-4,-5]
invert([1,-2,3,-4,5]) == [-1,2,-3,4,-5]
invert([]) == []

https://www.codewars.com/kata/5899dc03bc95b1bf1b0000ad/train/rust
*/