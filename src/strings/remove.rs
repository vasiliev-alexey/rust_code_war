use itertools::Itertools;

fn remove(s: &str) -> String {

      let mut res = vec![];

    for x in s.split_whitespace() {

        if x.chars().filter(|c| c == &'!').count() !=1 {
            res.push(x);
        }

    }
    res.iter().join(" ")
}


// Add your tests here.
// See https://doc.rust-lang.org/stable/rust-by-example/testing/unit_testing.html

#[cfg(test)]
mod tests {
    use super::remove;

    #[test]
    fn fixed_tests() {
        assert_eq!(remove("Hi!"), String::from(""));
        assert_eq!(remove("Hi! Hi!"), String::from(""));
        assert_eq!(remove("Hi! Hi! Hi!"), String::from(""));
        assert_eq!(remove("Hi Hi! Hi!"), String::from("Hi"));
        assert_eq!(remove("Hi! !Hi Hi!"), String::from(""));
        assert_eq!(remove("Hi! Hi!! Hi!"), String::from("Hi!!"));
        assert_eq!(remove("Hi! !Hi! Hi!"), String::from("!Hi!"));
        assert_eq!(remove("Hi! !Hi! Hi! Hi"), String::from("!Hi! Hi"));
    }
}
