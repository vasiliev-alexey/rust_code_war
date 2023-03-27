fn switcher(numbers: Vec<&str>) -> String {
    numbers.iter().map(|&s| {
        if s == "27" { return '!'; }
        if s == "28" { return '?'; }
        if s == "29" { return ' '; }
        return (('a' as u8) + (26 - s.parse::<u8>().unwrap())) as char;
    }).collect()
}

// Add your tests here.
// See https://doc.rust-lang.org/stable/rust-by-example/testing/unit_testing.html

#[cfg(test)]
mod tests {
    use super::switcher;

    #[test]
    fn example_tests() {
        assert_eq!(switcher(vec!["24", "12", "23", "22", "4", "26", "9", "8"]), "codewars");
        assert_eq!(switcher(vec!["25","7","8","4","14","23","8","25","23","29","16","16","4"]), "btswmdsbd kkw");
        assert_eq!(switcher(vec!["4", "24"]), "wc");
    }
}


/*
Given an array of numbers (in string format), you must return a string. The numbers correspond to the letters of the alphabet in reverse order:
 a=26, z=1 etc. You should also account for '!', '?' and ' ' that are represented by '27', '28' and '29' respectively.
All inputs will be valid.
*/