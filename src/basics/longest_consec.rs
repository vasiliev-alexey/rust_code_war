fn longest_consec(strarr: Vec<&str>, k: usize) -> String {
    let mut long_str = String::from("");

    if (strarr.len() == 0) || (k <= 0) || (k > strarr.len()) {
        return long_str;
    }
    for i in 0..strarr.len() - k + 1 {
        let mut str = String::from("");
        for r in 0..k {
            str.push_str(strarr[i + r]);
        }

        if str.len() > long_str.len() {
            long_str = str.clone();
        }
    }
    long_str
}

/*
fn longest_consec(strarr: Vec<&str>, k: usize) -> String {
    if k > strarr.len() || k == 0 || strarr.len() == 0 { String::new() } else {
        strarr.windows(k).map(|x| { x.join("") }).rev().max_by_key(String::len).unwrap()
    }
}
*/


fn testing(strarr: Vec<&str>, k: usize, exp: &str) -> () {
    assert_eq!(&longest_consec(strarr, k), exp)
}

#[test]
fn basics_longest_consec() {
    testing(vec!["zone", "abigail", "theta", "form", "libe", "zas"], 2, "abigailtheta");
    testing(vec!["ejjjjmmtthh", "zxxuueeg", "aanlljrrrxx", "dqqqaaabbb", "oocccffuucccjjjkkkjyyyeehh"], 1,
            "oocccffuucccjjjkkkjyyyeehh");
    testing(vec![], 3, "");
    testing(vec!["it", "wkppv", "ixoyx", "3452", "zzzzzzzzzzzz"], 3, "ixoyx3452zzzzzzzzzzzz");
    testing(vec!["it", "wkppv", "ixoyx", "3452", "zzzzzzzzzzzz"], 15, "");
    testing(vec!["it", "wkppv", "ixoyx", "3452", "zzzzzzzzzzzz"], 0, "");
}

/*
You are given an array(list) strarr of strings and an integer k. Your task is to return the first longest string consisting of k consecutive strings taken in the array.

Examples:
strarr = ["tree", "foling", "trashy", "blue", "abcdef", "uvwxyz"], k = 2

Concatenate the consecutive strings of strarr by 2, we get:

treefoling   (length 10)  concatenation of strarr[0] and strarr[1]
folingtrashy ("      12)  concatenation of strarr[1] and strarr[2]
trashyblue   ("      10)  concatenation of strarr[2] and strarr[3]
blueabcdef   ("      10)  concatenation of strarr[3] and strarr[4]
abcdefuvwxyz ("      12)  concatenation of strarr[4] and strarr[5]

Two strings are the longest: "folingtrashy" and "abcdefuvwxyz".
The first that came is "folingtrashy" so
longest_consec(strarr, 2) should return "folingtrashy".

In the same way:
longest_consec(["zone", "abigail", "theta", "form", "libe", "zas", "theta", "abigail"], 2) --> "abigailtheta"
n being the length of the string array, if n = 0 or k > n or k <= 0 return "" (return Nothing in Elm, "nothing" in Erlang).

Note
consecutive strings : follow one after another without an interruption

https://www.codewars.com/kata/56a5d994ac971f1ac500003e/solutions/rust
 */