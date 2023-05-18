fn collatz(n: u64) -> u64 {
    let mut tmp = n;

    let mut result: Vec<u64> = vec![];
    while tmp > 1 {
        if tmp % 2 == 0 {
            tmp = tmp / 2;
            result.push(tmp)
        } else {
            tmp = (tmp * 3) + 1;
            result.push(tmp);
        };
    };
    result.push(1);
    return result.len() as u64;
}


#[cfg(test)]
mod tests {
    use super::collatz;

    fn dotest(n: u64, expected: u64) {
        let actual = collatz(n);
        assert!(actual == expected,
                "With n = {n}\nExpected {expected} but got {actual}")
    }

    #[test]
    fn fixed_tests() {
        dotest(20, 8);
        dotest(15, 18);
    }
}


/*
The Collatz Conjecture states that for any positive natural number n, this process:

    if n is even, divide it by 2
    if n is odd, multiply it by 3 and add 1
    repeat
will eventually reach n = 1.
For example, if n = 20, the resulting sequence will be:

[ 20, 10, 5, 16, 8, 4, 2, 1 ]
Write a program that will output the length of the Collatz Conjecture for any given n.
In the example above, the output would be 8.

For more reading see: http://en.wikipedia.org/wiki/Collatz_conjecture
*/