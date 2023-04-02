use itertools::Itertools;

fn remove_parentheses(s: &str) -> String {
    let mut res = vec![];
    let mut open = 0u8;
    for c in s.chars() {
        match c {
            '(' => open += 1,
            ')' => open -= 1,
            _ => if open == 0 { res.push(c) }
        }
    }
    return res.iter().join("");
}

#[cfg(test)]
mod tests {
    use super::remove_parentheses;

    #[test]
    fn sample_tests() {
        assert_eq!(remove_parentheses("example(unwanted thing)example"), "exampleexample");
        assert_eq!(remove_parentheses("example (unwanted thing) example"), "example  example");
        assert_eq!(remove_parentheses("a (bc d)e"), "a e");
        assert_eq!(remove_parentheses("a(b(c))"), "a");
        assert_eq!(remove_parentheses("hello example (words(more words) here) something"), "hello example  something");
        assert_eq!(remove_parentheses("(first group) (second group) (third group)"), "  ");
    }
}
