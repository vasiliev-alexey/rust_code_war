fn sum_of_differences(arr: &[i8]) -> Option<i8> {

    if arr.is_empty() || arr.len() == 1 { return None; }
    return Some(arr.iter().max().unwrap() - arr.iter().min().unwrap());

}



// Add your tests here.
// See https://doc.rust-lang.org/stable/rust-by-example/testing/unit_testing.html

#[cfg(test)]
mod tests {
    use super::sum_of_differences;


    const ERR_MSG: &str = "\nYour result (left) did not match the expected output (right)";
    #[test]
    fn sample_tests() {
        assert_eq!(sum_of_differences(&[1, 2, 10]), Some(9), "{}", ERR_MSG);
         assert_eq!(sum_of_differences(&[-3, -2, -1]), Some(2), "{}", ERR_MSG);
         assert_eq!(sum_of_differences(&[1, 1, 1, 1, 1]), Some(0), "{}", ERR_MSG);
         assert_eq!(sum_of_differences(&[-17, 17]), Some(34), "{}", ERR_MSG);
         assert_eq!(sum_of_differences(&[]), None, "{}", ERR_MSG);
         assert_eq!(sum_of_differences(&[0]), None, "{}", ERR_MSG);
        assert_eq!(sum_of_differences(&[-1]), None, "{}", ERR_MSG);
        assert_eq!(sum_of_differences(&[1]), None, "{}", ERR_MSG);
        assert_eq!(sum_of_differences(&[1, 0, 1, 0, 0, -1, -1, -1, 0]), Some(2), "{}", ERR_MSG);
    }
}


/*
Sum of differences in array
Your task is to sum the differences between consecutive pairs in the array in descending order.
Example
[2, 1, 10]  -->  9
In descending order: [10, 2, 1]
Sum: (10 - 2) + (2 - 1) = 8 + 1 = 9
If the array is empty or the array has only one element the result should be 0 (Nothing in Haskell, None in Rust).
*/