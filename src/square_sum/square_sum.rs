fn square_sum(vec: Vec<i32>) -> i32 {
    let mut i: i32 = 0;
    for x in vec {
        i = i + (x * x);
    }
    i
    //vec.iter().map(|s| s * s).sum()
}

#[test]
fn returns_expected() {
    assert_eq!(square_sum(vec![1, 2]), 5);
    assert_eq!(square_sum(vec![-1, -2]), 5);
    assert_eq!(square_sum(vec![5, 3, 4]), 50);
    assert_eq!(square_sum(vec![]), 0);
}