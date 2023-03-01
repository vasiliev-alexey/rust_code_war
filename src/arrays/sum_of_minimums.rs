/*
Given a string of words (x), you need to return an array of the words, sorted alphabetically by the final character in each.
If two words have the same last letter, they returned array should show them in the order they appeared in the given string.
All inputs will be valid.
*/


fn sum_of_minimums(numbers: [[u8; 4]; 4]) -> u8 {
    numbers.iter().map( |x|  x.iter().min().unwrap()).sum()

}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sum_of_minimums() {
        assert_eq!(sum_of_minimums([[7, 9, 8, 6], [6, 5, 4, 3], [5, 7, 4, 5], [7, 9, 4, 3]]), 16);
        assert_eq!(sum_of_minimums([[7, 9, 8, 6], [6, 5, 4, 3], [5, 7, 4, 5], [7, 9, 4, 4]]), 17);
        assert_eq!(sum_of_minimums([[7, 9, 8, 84], [6, 5, 4, 65], [5, 7, 4, 23], [7, 9, 4, 25]]), 19);
    }
}
