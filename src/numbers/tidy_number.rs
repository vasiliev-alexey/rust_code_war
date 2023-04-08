use itertools::Itertools;

fn tidy_number(n: u64) -> bool {
    n.to_string().chars().map( |c| c.to_digit(10).unwrap()).tuple_windows().filter( | (x, y)|  x > y).count() == 0
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn basic() {
        assert_eq!(tidy_number(12), true);
        assert_eq!(tidy_number(102), false);
        assert_eq!(tidy_number(9672), false);
        assert_eq!(tidy_number(2789), true);
        assert_eq!(tidy_number(2335), true);
    }
}
