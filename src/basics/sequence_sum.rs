fn sequence_sum(start: u32, end: u32, step: u32) -> u32 {
    let mut  rez = 0;


    for x in (start..end+1).step_by(step as usize) {
        rez = rez + x;
    }
    return rez;
}



#[cfg(test)]
mod tests {
    use super::sequence_sum;

    fn dotest(a: u32, b: u32, c: u32, expected: u32) {
        let actual = sequence_sum(a, b, c);
        assert!(actual == expected,
                "With start = {a}, end = {b}, step = {c}\nExpected {expected} but got {actual}")
    }

    #[test]
    fn fixed_tests() {
        dotest(2, 6, 2, 12);
        dotest(1, 5, 1, 15);
        dotest(1, 5, 3, 5);
        dotest(0, 15, 3, 45);
        dotest(16, 15, 3, 0);
        dotest(2, 24, 22, 26);
        dotest(2, 2, 2, 2);
        dotest(2, 2, 1, 2);
        dotest(1, 15, 3, 35);
        dotest(15, 1, 3, 0);
    }
}
// https://www.codewars.com/kata/586f6741c66d18c22800010a/train/rust

/* Sum of a sequence
Your task is to write a function which returns the sum of a sequence of integers.

The sequence is defined by 3 non-negative values: begin, end, step.

If begin value is greater than the end, your function should return 0. If end is not the result of an integer number of steps, then don't add it to the sum. See the 4th example below.

Examples

2,2,2 --> 2
2,6,2 --> 12 (2 + 4 + 6)
1,5,1 --> 15 (1 + 2 + 3 + 4 + 5)
1,5,3  --> 5 (1 + 4)
This is the first kata in the series:
*/