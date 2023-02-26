use itertools::Itertools;

fn reverse_letters(s: &str) -> String {
    s.chars().filter(|x| x.is_alphabetic()).rev().join("").to_string()
}


#[cfg(test)]
mod tests {
    use super::reverse_letters;

    fn dotest(s: &str, expected: &str) {
        let actual = reverse_letters(s);
        assert!(actual == expected,
                "With s = \"{s}\"\nExpected \"{expected}\" but got \"{actual}\"")
    }

    #[test]
    fn fixed_tests() {
        dotest("krishan","nahsirk");
        dotest("ultr53o?n","nortlu");
        dotest("ab23c","cba");
        dotest("krish21an","nahsirk");
    }
}
