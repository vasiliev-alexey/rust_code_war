

fn power_of_two(x: u64) -> bool {
 // x > 0 && 2u64.pow(x.checked_ilog(2).unwrap() )==x
    if x == 0 {
        false
    } else {
        let mut exponent: u32 = 0;
        loop {
            let current: u64 = 2_u64.pow(exponent);
            if current == x {
                break true;
            } else if current > x {
                break false;
            } else {
                exponent += 1;
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn basic_tests() {
        assert_eq!(power_of_two(0), false);
        assert_eq!(power_of_two(2), true);
        assert_eq!(power_of_two(5), false);
        assert_eq!(power_of_two(6), false);
        assert_eq!(power_of_two(8), true);
        assert_eq!(power_of_two(1024), true);
        assert_eq!(power_of_two(4096), true);
        assert_eq!(power_of_two(8191), false);
    }
}