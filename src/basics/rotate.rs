use itertools::Itertools;

fn rotate(s: &str) -> Vec<String> {
    let mut res = vec![];
    match s.len() {
        0 => res,
        1 => vec![s.to_string()],
        _ => {
            let first = s.chars().next().unwrap().to_string();
            let rest = s.chars().skip(1).collect_vec();
            for i in 0..  rest.len()+1{
                let tmp = rest[i..].iter().join("") + &*first + rest[0..i].iter().join("").as_str();
                res.push(tmp);
            }
            res
        }
    }
}

// Add your tests here
// https://doc.rust-lang.org/book/ch11-01-writing-tests.html

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_rotate_empty() {
        assert_eq!(rotate(""), Vec::<String>::new(), "Should return empty string for input: {:?}", "");
    }

    #[test]
    fn test_rotate_single() {
        assert_eq!(rotate("a"), vec!["a"], "\n\nYour result (left) did not match the expected output (right) for the input: {:?}", "a");
    }

    #[test]
    fn test_rotate_triple() {
        assert_eq!(rotate("abc"), vec!["bca", "cab", "abc"],
                   "\n\nYour result (left) did not match the expected output (right) for the input: {:?}", "abc");
    }

    #[test]
    fn test_rotate_long() {
        assert_eq!(rotate("hello"), vec!["elloh", "llohe", "lohel", "ohell", "hello"],
                   "\n\nYour result (left) did not match the expected output (right) for the input: {:?}", "hello");
    }
}
