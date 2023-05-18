fn find_deleted_number(list: &[u16], mixed_list: &[u16]) -> Option<u16> {
    for x in list {
            if !mixed_list.contains(x) {
                return Some(*x)
            }
    }
    None
}


#[cfg(test)]
mod tests {
    use super::find_deleted_number;

    #[test]
    fn basic() {
        assert_eq!(
            find_deleted_number(&[1, 2, 3, 4, 5, 6, 7, 8, 9], &[3, 2, 4, 6, 7, 8, 1, 9]),
            Some(5)
        );
        assert_eq!(
            find_deleted_number(&[1, 2, 3, 4, 5, 6, 7, 8, 9], &[3, 2, 4, 6, 7, 8, 9, 5]),
            Some(1)
        );
        assert_eq!(
            find_deleted_number(&[1, 2, 3, 4, 5, 6, 7, 8, 9], &[3, 2, 4, 1, 7, 8, 9, 5]),
            Some(6)
        );
    }
}