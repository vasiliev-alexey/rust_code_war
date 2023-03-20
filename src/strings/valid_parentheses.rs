fn valid_parentheses(parens: &str) -> bool {
    let mut count = 0;
    for c in parens.chars() {
        match c {
            '(' => count += 1,
            ')' => count -= 1,
            _ => (),
        };
        if count < 0 {
            return false;
        }
    }
    count == 0
}


// Add your tests here.
// See https://doc.rust-lang.org/stable/rust-by-example/testing/unit_testing.html

#[cfg(test)]
mod tests {
    use super::*;

    fn do_test(expected: bool, input: &str) {
        assert_eq!(valid_parentheses(input), expected, "\nYour result (left) did not match the expected output (right) for the input: {input:?}");
    }

    #[test]
    fn valid_cases() {
        do_test(true, "()");
        do_test(true, "((()))");
        do_test(true, "()()()");
        do_test(true, "(()())()");
        do_test(true, "()(())((()))(())()");
    }

    #[test]
    fn invalid_cases()  {
        do_test(false, ")(");
        do_test(false, "()()(");
        do_test(false, "((())");
        do_test(false, "())(()");
        do_test(false, ")()");
        do_test(false, ")");
    }

    #[test]
    fn empty_string() {
        do_test(true, "");
    }
}


/* Valid Parentheses
https://www.codewars.com/kata/6411b91a5e71b915d237332d/train/rust
Write a function that takes a string of parentheses, and determines if the order of the parentheses is valid. The function should return true if the string is valid, and false if it's invalid.
Examples

"()"              =>  true
")(()))"          =>  false
"("               =>  false
"(())((()())())"  =>  true

Constraints

0 <= length of input <= 100

    All inputs will be strings, consisting only of characters ( and ).
    Empty strings are considered balanced (and therefore valid), and will be tested.
    For languages with mutable strings, the inputs should not be mutated.


Protip: If you are trying to figure out why a string of parentheses is invalid, paste the parentheses into the code editor, and let the code highlighting show you!

*/