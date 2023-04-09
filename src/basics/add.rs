fn add(args: &[i64]) -> i64 {
    return if args.len()==0 {0} else {args.iter().enumerate().fold( 0, |a , (i , b)|  a + ( b*(i as i64 +1)))}
}

// Rust test example:
// TODO: replace with your own tests (TDD), these are just how-to examples.
// See: https://doc.rust-lang.org/book/testing.html
//https://www.codewars.com/kata/555b73a81a6285b6ce000047/train/rust
#[test]
fn basic_tests() {
    assert_eq!(add(&[]), 0);
    assert_eq!(add(&[4,-3,-2]), -8);
}