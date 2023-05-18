

fn shifted_diff(first: &str, second: &str) -> Option<usize> {
    if first.len() != second.len()
    { return None; }
    second.repeat(2).find(first)


}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example() {
        assert_eq!(shifted_diff("eecoff", "coffee"), Some(4));
        assert_eq!(shifted_diff("Moose", "moose"), None);
        assert_eq!(shifted_diff("isn't", "'tisn"), Some(2));
        assert_eq!(shifted_diff("Esham", "Esham"), Some(0));
        assert_eq!(shifted_diff(" ", " "), Some(0));
        assert_eq!(shifted_diff("hoop", "pooh"), None);
        assert_eq!(shifted_diff("  ", " "), None);
    }
}

