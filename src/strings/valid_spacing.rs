fn valid_spacing(s: &str) -> bool {
    return s.trim() == s && !s.contains("  ");
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn samples() {
        assert_eq!(valid_spacing("Hello world"), true, "Testing 'Hello world'");
        assert_eq!(valid_spacing(" Hello world"), false, "Testing ' Hello world'");
        assert_eq!(valid_spacing("Hello  world "), false, "Testing 'Hello  world '");
        assert_eq!(valid_spacing("Hello"), true, "Testing 'Hello'");
        assert_eq!(valid_spacing("Helloworld"), true, "Testing 'Helloworld'");
    }
}

