use itertools::Itertools;

fn contamination(text: &str, character: &str) -> String {
    text.chars().map(|_c| character).join("")
}


// Add your tests here.
// See https://doc.rust-lang.org/stable/rust-by-example/testing/unit_testing.html

#[cfg(test)]
mod tests {
    use super::contamination;

    fn dotest(a: &str, b: &str, expected: &str) {
        let actual = contamination(a, b);
        assert!(actual == expected,
                "With text = \"{a}\", character = \"{b}\"\nExpected \"{expected}\" but got \"{actual}\"")
    }

    #[test]
    fn fixed_tests() {
        dotest("abc","z", "zzz");
        dotest("","z", "");
        dotest("abc","", "");
        dotest("_3ebzgh4","&", "&&&&&&&&");
        dotest("//case"," ", "      ");
    }
}
