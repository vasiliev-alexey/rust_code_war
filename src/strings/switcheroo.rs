use itertools::Itertools;

fn switcheroo(s: &str) -> String {
    let mut res = Vec::with_capacity(s.len());
    for x in s.chars() {

        if x == 'a' {
            res.push('b');
        }
        else if x == 'b' {
            res.push('a');
        } else {
            res.push(x);
        }

    }
    res.iter().join("")

}


// Add your tests here.
// See https://doc.rust-lang.org/stable/rust-by-example/testing/unit_testing.html

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_add() {
        assert_eq!(switcheroo("abc"), "bac");
        assert_eq!(switcheroo("aaabcccbaaa"), "bbbacccabbb");
        assert_eq!(switcheroo("ccccc"), "ccccc");
        assert_eq!(switcheroo("abababababababab"), "babababababababa");
        assert_eq!(switcheroo("aaaaa"), "bbbbb");
    }
}
