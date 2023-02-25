fn solve(s: &str) -> String {
    match s.chars().filter(|x| x.is_lowercase()).count() >= s.len() / 2 {
        true => s.to_lowercase(),
        false => s.to_uppercase(),
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn basic() {
        assert_eq!(solve("code"), "code");
        assert_eq!(solve("CODe"), "CODE");
        assert_eq!(solve("COde"), "code");
        assert_eq!(solve("Code"), "code");
    }
}