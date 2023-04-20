fn quadrant(x: i32, y: i32) -> i32 {
    match   (y > 0 , x > 0) {
        (true , true) => 1,
        (true , false) => 2,
        (false , false) => 3,
        (false , true) => 4,
        _ => unreachable!()

    }
}

//https://www.codewars.com/kata/643af0fa9fa6c406b47c5399/train/rust


// Add your tests here.
// See https://doc.rust-lang.org/stable/rust-by-example/testing/unit_testing.html

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn fixed_tests() {
        assert_eq!(quadrant(1, 2), 1);
        assert_eq!(quadrant(3, 5), 1);
        assert_eq!(quadrant(-10, 100), 2);
        assert_eq!(quadrant(-1, -9), 3);
        assert_eq!(quadrant(19, -56), 4);    }
}
