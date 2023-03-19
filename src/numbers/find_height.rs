// #![no_std]
//
// use num_integer::Roots;

pub fn find_height(cubes: usize) -> u16 {
    let res = ((6 * cubes)as f64).cbrt() as u16;
    res - (6 * cubes < res as usize * (res as usize + 1) * (res as usize + 2)) as u16
}

// Add your tests here.
// See https://doc.rust-lang.org/stable/rust-by-example/testing/unit_testing.html

#[cfg(test)]
mod tests {
    use super::find_height;

    #[test]
    fn sample_tests() {
        assert_eq!(find_height(0), 0);
        assert_eq!(find_height(1), 1);
        assert_eq!(find_height(3), 1);
        assert_eq!(find_height(4), 2);
        assert_eq!(find_height(10), 3);
    }
}
