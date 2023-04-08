use std::ops::Add;
use num::BigUint;
use regex::Regex;

fn increment_string(s: &str) -> String {
    let r = Regex::new(r"\d*$").unwrap();

    let sp = r.find(s).unwrap();
    let num_len = sp.end() - sp.start();
    if num_len == 0 {
        return s.to_string() + "1";
    }
    let digs = &s[sp.start()..];
    let next_num = digs.parse::<BigUint>().unwrap().add(BigUint::from(1u8));
    return format!("{}{:0width$}", s[0..sp.start()].to_string(), next_num, width = num_len);
}


// Add your tests here.
// See https://doc.rust-lang.org/stable/rust-by-example/testing/unit_testing.html
// https://www.codewars.com/kata/54a91a4883a7de5d7800009c/train/rust
#[cfg(test)]
mod tests {
    use super::increment_string;

    fn dotest(s: &str, expected: &str) {
        let actual = increment_string(s);
        assert!(actual == expected,
                "Test failed with s = \"{s}\"\nExpected \"{expected}\"\nBut got \"{actual}\"")
    }

    #[test]
    fn sample_tests() {
        dotest("", "1");
        dotest("foo", "foo1");
        dotest("009", "010");
        dotest("TJbQOiGdGElwbYmv0IaQ8M5GCIBBEhbCk2tCMfTEHuv6ZKIBvuNv1hrEH99999999999999999999999999999999999999999999999999999999999999999999999999999999999999", "010");
        dotest("HereComesATrickyTest99999999999999999999999999999999999999", "010");
        dotest("foobar001", "foobar002");


        dotest("foobar1", "foobar2");
        dotest("foobar00", "foobar01");
        dotest("foobar99", "foobar100");
        dotest("foobar099", "foobar100");
    }
}
