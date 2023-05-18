fn cockroach_speed(s: f64) -> i64
{
    (s * 100000.0 / 3600.0) as i64
}

// https://www.codewars.com/kata/55fab1ffda3e2e44f00000c6/train/rust
#[test]
fn basic_tests() {
    assert_eq!(cockroach_speed(1.08), 30);
    assert_eq!(cockroach_speed(1.09), 30);
    assert_eq!(cockroach_speed(0.0), 0);
}