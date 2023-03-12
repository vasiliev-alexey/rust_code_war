use itertools::Itertools;

fn multiply(a: &str, b: &str) -> String {
    if a == "0" || b == "0" {
        return String::from("0");
    }

    let mut rez = vec![];

    for _ in 0..(a.len() + b.len()) {
        rez.push(0);
    }


    let mut rez_size = rez.len() - 1;


    let first_str = if a.len() > b.len() { a } else { b };
    let second_str = if a.len() > b.len() { b } else { a };

    let first = first_str.split("").filter(|x| x.len() > 0).map(|x| x.parse::<u8>().unwrap_or(0)).collect::<Vec<_>>();
    let second = second_str.split("").filter(|x| x.len() > 0).map(|x| x.parse::<u8>().unwrap_or(0)).collect::<Vec<_>>();

    for (i, _) in first.iter().enumerate().rev() {
        let mut curr_idx = rez_size;
        rez_size = rez_size - 1;
        for (j, _) in second.iter().enumerate().rev() {
            let val = first[i] * second[j] + rez[curr_idx];
            rez[curr_idx] = val % 10;

            curr_idx = curr_idx - 1;
            rez[curr_idx] += val / 10;
        }
    }


    rez.iter().join("").trim_start_matches("0").to_string()
}

// Add your tests here.
// See https://doc.rust-lang.org/stable/rust-by-example/testing/unit_testing.html

#[cfg(test)]
mod sample_tests {
    use super::multiply;

    fn do_test(a: &str, b: &str, expected: &str) {
        let actual = multiply(&a, &b);
        assert_eq!(actual, expected,
                   "\n\nMultiplying a*b with\na = {a}\nb = {b}\nshould return: {expected}\ninstead got: {actual}");
    }

    #[test]
    fn simple_cases() {
        //        input       expected
        do_test("0", "3", "0");
        do_test("2", "3", "6");
        do_test("30", "69", "2070");
        do_test("11", "85", "935");
    }

    #[test]
    fn edge_cases() {
        do_test("2", "0", "0");
        do_test("0", "30", "0");
        do_test("0000001", "3", "3");
        do_test("1009", "03", "3027");
    }

    #[test]
    fn big_numbers() {
        do_test("98765", "56894", "5619135910");
        do_test("9007199254740991", "9007199254740991", "81129638414606663681390495662081");
        do_test("1020303004875647366210", "2774537626200857473632627613",
                "2830869077153280552556547081187254342445169156730");
        do_test("58608473622772837728372827", "7586374672263726736374",
                "444625839871840560024489175424316205566214109298");
    }
}

/* Multiplying numbers as strings
https://www.codewars.com/kata/55911ef14065454c75000062/train/rust
This is the first part. You can solve the second part here when you are done with this. Multiply two numbers! Simple!

The arguments are passed as strings.
The numbers may be way very large
Answer should be returned as a string
The returned "number" should not start with zeros e.g. 0123 is invalid
Note: 100 randomly generated tests!

Important
Usage of num::BigInt is disallowed and will be checked in the full test suite.
*/