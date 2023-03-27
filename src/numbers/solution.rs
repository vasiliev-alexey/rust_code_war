fn solution(num: i32) -> i32 {
    let mut rez = 0;
    for i in 0.. num{
        if i % 3 == 0 || i % 5 == 0 {
            rez = rez +i;
        }

    }
    rez

}

mod tests {
    use super::solution;

    #[test]
    fn sample_tests() {
        // assertion(expected, input);
        assertion(  23,   10);
        assertion(  33,   11);
        assertion( 225,   33);
        assertion(   8,    6);
        assertion(3420,  123);
        assertion( 543,   50);
        assertion(   0,    0);
        assertion(   0, -203);
        assertion(25719750, 10500);
    }

    fn assertion(expected : i32, input : i32) {
        let actual = solution(input);

        assert!(
            expected == actual,
            "\nTest failed!\n expected: {}\n actual: {}\n input: {}\n", expected, actual, input
        );
    }
}

/*
If we list all the natural numbers below 10 that are multiples of 3 or 5, we get 3, 5, 6 and 9. The sum of these multiples is 23.
Finish the solution so that it returns the sum of all the multiples of 3 or 5 below the number passed in. Additionally, if the number is negative, return 0 (for languages that do have them).
Note: If the number is a multiple of both 3 and 5, only count it once.
*/