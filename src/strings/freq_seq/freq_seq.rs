use std::collections::HashMap;

fn freq_seq(s: &str, sep: &str) -> String {

    let mut hs = HashMap::new();

    for x in s.chars() {
        *hs.entry(x).or_insert(0) += 1;
    }
    s.chars().map( |x| hs.get(&x).unwrap().to_string()).collect::<Vec<_>>().join(sep)
}



#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn returns_expected() {
        assert_eq!(freq_seq("hello world", "-"), "1-1-3-3-2-1-1-2-1-3-1");
        // assert_eq!(freq_seq("19999999", ":"), "1:7:7:7:7:7:7:7");
        // assert_eq!(freq_seq("^^^**$", "x"), "3x3x3x2x2x1");
    }
}
