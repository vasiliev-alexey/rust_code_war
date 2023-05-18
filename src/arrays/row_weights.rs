fn row_weights(array: Vec<u32>) -> (u32, u32) {

    let left = array.iter().enumerate().filter(| (i , _) | i%2 ==0 ).map(|x| x.1).sum();
    let right = array.iter().enumerate().filter(| (i , _) | i%2 ==1 ).map(|x| x.1).sum();
    (left , right)

}

//https://www.codewars.com/kata/5abd66a5ccfd1130b30000a9/train/rust


#[test]
fn basic_tests() {
    assert_eq!(row_weights(vec![13, 27, 49]), (62, 27));
    assert_eq!(row_weights(vec![50, 60, 70, 80]), (120, 140));
    assert_eq!(row_weights(vec![80]), (80,0));
}