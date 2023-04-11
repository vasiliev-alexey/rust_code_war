use itertools::Itertools;

fn min_value(digits: Vec<i32>) -> i32 {
    let arr = digits.iter().unique().sorted_by(|x, y| x.cmp(y)).map(|x| *x).collect_vec();
    arr.iter().join("").parse::<i32>().unwrap()
}

// https://www.codewars.com/kata/5ac6932b2f317b96980000ca/train/rust
#[test]
fn basic_test() {
    assert_eq!(min_value(vec![1, 3, 1]), 13);
    assert_eq!(min_value(vec![4, 7, 5, 7]), 457);
    assert_eq!(min_value(vec![4, 8, 1, 4]), 148);
}