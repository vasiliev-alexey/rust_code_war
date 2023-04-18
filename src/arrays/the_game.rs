pub fn the_game(frank: &[u8; 4], sam: &[u8; 4], tom: &[u8; 4]) -> bool {
    [sam, tom]
        .into_iter()
        .all(|p| frank.iter().skip(2).zip(p).all(|(f, p)| f > p))
}

mod tests {
    use super::*;

    #[test]
    fn good_day() {
        assert_eq!(the_game(&[2, 5, 8, 11], &[1, 4, 7, 10], &[0, 3, 6, 9]), true);
    }

    #[test]
    fn bad_day() {
        assert_eq!(the_game(&[1, 2, 3, 4], &[5, 6, 7, 8], &[0, 9, 10, 11]), false);
    }
}