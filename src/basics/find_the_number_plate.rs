use itertools::Itertools;
use num::Integer;

fn find_the_number_plate(n: u32) -> String {
    let (left_count, right_count) = (n).div_mod_floor(&999);
    let left_part = || {
        let alphabet = "abcdefghijklmnopqrstuvwxyz";
        let alphabet_len = alphabet.len() as u32;
        let counters = vec![left_count % alphabet.len() as u32, (left_count / alphabet_len) % alphabet_len,
                            (left_count / alphabet_len.pow(2)) % alphabet_len];
        counters.iter().map(|x| alphabet.chars().nth(*x as usize).unwrap()).join("")
    };
    let left = left_part();
    let right = format!("{:0>3}", (right_count + 1).to_string());
    (left + right.as_str()).to_string()
}

// Add your tests here.
// See https://doc.rust-lang.org/stable/rust-by-example/testing/unit_testing.html

#[cfg(test)]
mod tests {
    use super::find_the_number_plate;

    fn dotest(n: u32, expected: &str) {
        let actual = find_the_number_plate(n);
        assert!(actual == expected,
                "With n = {n}\nExpected \"{expected}\" but got \"{actual}\"")
    }

    #[test]
    fn fixed_tests() {
        dotest(3, "aaa004");
        dotest(1487, "baa489");
        dotest(40000, "oba041");
        dotest(17558423, "zzz999");
        dotest(43056, "rba100");
        dotest(234567, "aja802");
    }
}
