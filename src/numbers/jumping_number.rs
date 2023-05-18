use itertools::Itertools;


fn jumping_number(n: u64) -> String {
    let res = n.to_string().chars().map(|x| x.to_digit(10).unwrap() as i32).tuple_windows().filter(|(x, y)| (x - y).abs() != 1).collect_vec().len() > 0;
    return if res { "Not!!".to_string() } else { "Jumping!!".to_string() };
}

// https://www.codewars.com/kata/5a54e796b3bfa8932c0000ed/train/rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn basic() {
        assert_eq!(jumping_number(1), "Jumping!!");
        assert_eq!(jumping_number(7), "Jumping!!");
        assert_eq!(jumping_number(9), "Jumping!!");
        assert_eq!(jumping_number(23), "Jumping!!");
        assert_eq!(jumping_number(32), "Jumping!!");

        assert_eq!(jumping_number(98), "Jumping!!");
        assert_eq!(jumping_number(8987), "Jumping!!");
        assert_eq!(jumping_number(4343456), "Jumping!!");
        assert_eq!(jumping_number(98789876), "Jumping!!");
        assert_eq!(jumping_number(79), "Not!!");
    }
}
