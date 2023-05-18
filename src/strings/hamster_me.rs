use std::collections::HashMap;
use itertools::Itertools;


fn hamster_me(code: &str, message: &str) -> String {
    let codes = code.chars().unique().sorted_by(|x, y| x.cmp(y)).collect_vec();
    let mut num_pos = 1u8;
    let mut code_char = *codes.last().unwrap();
    let first_char = codes[0];
    let mut map_al = HashMap::new();

    let mut alp = vec![];
    for _ in 0..2 {
        for i in 97..123 {
            alp.push(i as u8 as char)
        }
    }
    let index = alp.iter().position(|&r| r == first_char).unwrap();

    for i in index..(index + 26) {
        if codes.contains(&alp[i]) {
            num_pos = 1;
            code_char = alp[i];
        }
        map_al.insert(alp[i], format!("{code_char}{num_pos}"));
        num_pos += 1;
    }
    return message.chars().map(|c| map_al.get(&c).unwrap()).join("").to_string();
}


// Add your tests here.
// See https://doc.rust-lang.org/stable/rust-by-example/testing/unit_testing.html

#[cfg(test)]
mod tests {
    use super::hamster_me;

    fn dotest(code: &str, message: &str, expected: &str) {
        let actual = hamster_me(code, message);
        assert!(actual == expected,
                "With code = \"{code}\", message = \"{message}\"\nExpected \"{expected}\" but got \"{actual}\"")
    }

    #[test]
    fn fixed_tests() {
        dotest("hmster", "hamster", "h1t8m1s1t1e1r1");
        dotest("hamster", "hamster", "h1a1m1s1t1e1r1");
        dotest("hamster", "helpme", "h1e1h5m4m1e1");

        dotest("hhhhammmstteree", "hamster", "h1a1m1s1t1e1r1");
        dotest("hamster", "abcdefghijklmnopqrstuvwxyz", "a1a2a3a4e1e2e3h1h2h3h4h5m1m2m3m4m5r1s1t1t2t3t4t5t6t7");
        dotest("f", "abcdefghijklmnopqrstuvwxyz", "f22f23f24f25f26f1f2f3f4f5f6f7f8f9f10f11f12f13f14f15f16f17f18f19f20f21");
    }
}
