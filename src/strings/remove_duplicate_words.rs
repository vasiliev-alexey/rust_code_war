use itertools::Itertools;

fn remove_duplicate_words(s: &str) -> String {

    s.split_whitespace().unique().join(" ")

}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sample_test_cases() {
        assert_eq!(remove_duplicate_words("alpha beta beta gamma gamma gamma delta alpha beta beta gamma gamma gamma delta"), "alpha beta gamma delta");
        assert_eq!(remove_duplicate_words("my cat is my cat fat"), "my cat is fat");
    }
}