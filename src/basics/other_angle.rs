fn other_angle(a: u32, b: u32) -> u32 {
    180 - a - b
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_tests() {
        assert_eq!(other_angle(30, 60), 90);
        assert_eq!(other_angle(60, 60), 60);
        assert_eq!(other_angle(43, 78), 59);
        assert_eq!(other_angle(10, 20), 150);
    }
}

/* https://www.codewars.com/kata/5a023c426975981341000014/train/rust
You are given two interior angles (in degrees) of a triangle.

Write a function to return the 3rd.

Note: only positive integers will be tested.
*/