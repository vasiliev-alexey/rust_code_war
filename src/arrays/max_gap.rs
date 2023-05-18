fn max_gap(xs: &[i32]) -> i32 {
    let mut arr = xs.to_vec();
    arr.sort();
    let  mut dif = 0i32;
    for i in 0.. arr.len()-1 {
        let x = arr.get(i).unwrap();
        let y = arr.get(i+1).unwrap();
        if y -x > dif {
            dif = y- x
        }
    }
    dif
}

//https://www.codewars.com/kata/5a7893ef0025e9eb50000013/train/rust
// https://doc.rust-lang.org/stable/rust-by-example/testing/unit_testing.html

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn check_positive_values() {
        assert_eq!(max_gap(&[13, 10, 2, 9, 5]), 4);
        assert_eq!(max_gap(&[13, 3, 5]), 8);
        assert_eq!(max_gap(&[24, 299, 131, 14, 26, 25]), 168);
    }

    #[test]
    fn check_negative_values() {
        assert_eq!(max_gap(&[-3, -27, -4, -2]), 23);
        assert_eq!(max_gap(&[-7, -42, -809, -14, -12]), 767);
    }

    #[test]
    fn check_mixed_values() {
        assert_eq!(max_gap(&[12, -5, -7, 0, 290]), 278);
        assert_eq!(max_gap(&[-54, 37, 0, 64, -15, 640, 0]), 576);
    }
}

