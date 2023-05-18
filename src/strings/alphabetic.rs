use itertools::Itertools;

fn alphabetic(s: &str) -> bool {
     s.chars().sorted_by(| x , y| x.cmp(y)).join("") == s

}


// Add your tests here.
// See https://doc.rust-lang.org/stable/rust-by-example/testing/unit_testing.html

#[cfg(test)]
mod tests {
    use super::alphabetic;

    fn dotest(s: &str, expected: bool) {
        let actual = alphabetic(s);
        assert!(actual == expected, "With s = \"{s}\"\nExpected {expected} but got {actual}")
    }

    #[test]
    fn fixed_tests() {
        dotest("asd", false);
        dotest("codewars", false);
        dotest("door", true);
        dotest("cell", true);
        dotest("z", true);
        dotest("", true);
    }
}
