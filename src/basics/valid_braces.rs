use std::collections::HashMap;

fn valid_braces(s: &str) -> bool {

    let mut braces_map = HashMap::new();
    braces_map.insert('(', ')');
    braces_map.insert('{', '}');
    braces_map.insert('[', ']');

     let  mut stack = vec![];

    for current_char in s.chars() {
        if braces_map.contains_key(&current_char) {
            stack.push(current_char);
        } else {
            if &current_char != braces_map.get(&stack.pop().unwrap_or('z')).unwrap_or(&'z') {
                return false;
            }
        }

    }
    return stack.len()  == 0
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn basic_tests() {
        expect_true("()");
        expect_false("[(])");
    }

    fn expect_true(s: &str) {
        assert!(valid_braces(s), "Expected {s:?} to be valid. Got false", s=s);
    }

    fn expect_false(s: &str) {
        assert!(!valid_braces(s), "Expected {s:?} to be invalid. Got true", s=s);
    }
}