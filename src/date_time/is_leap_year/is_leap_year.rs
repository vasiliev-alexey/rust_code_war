fn is_leap_year(year: i32) -> bool {
    year % 4 == 0 && (year % 400 == 0 || year % 100 != 0)
}

/*
fn is_leap_year(year: i32) -> bool {
    match year {
        x if x % 400 == 0 => true,
        x if x % 100 == 0 => false,
        x if x % 4 == 0 => true,
        _ => false,
    }
}
*/

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sample_tests() {
        assert_eq!(is_leap_year(1234), false);
        assert_eq!(is_leap_year(1984), true);
        assert_eq!(is_leap_year(2000), true);
        assert_eq!(is_leap_year(2010), false);
        assert_eq!(is_leap_year(2013), false);
    }
}