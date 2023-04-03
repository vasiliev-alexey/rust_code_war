fn is_valid_walk(walk: &[char]) -> bool {
    if walk.len() != 10 {
        return false;
    }
    let mut h = 0i8;
    let mut v = 0i8;
    for s in walk {
        match *s {
            's' => v = v + 1,
            'n' => v = v - 1,
            'w' => h = h + 1,
            'e' => h = h - 1,
            _ => unreachable!()
        }
    }
    v == 0i8 && h == 0i8
}
//https://www.codewars.com/kata/54da539698b8a2ad76000228/train/rust
// Add your tests here.
// See https://doc.rust-lang.org/stable/rust-by-example/testing/unit_testing.html

#[cfg(test)]
mod tests {
    use super::is_valid_walk;

    #[test]
    fn sample_tests() {
        assert!(is_valid_walk(&['n', 's', 'n', 's', 'n', 's', 'n', 's', 'n', 's']));
        assert!(!is_valid_walk(&['w', 'e', 'w', 'e', 'w', 'e', 'w', 'e', 'w', 'e', 'w', 'e']));
        assert!(!is_valid_walk(&['w']));
        assert!(!is_valid_walk(&['n', 'n', 'n', 's', 'n', 's', 'n', 's', 'n', 's']));
        assert!(!is_valid_walk(&['e', 'e', 'e', 'e', 'w', 'w', 's', 's', 's', 's']))
    }
}
