fn divisors(n: u32) -> u32 {
    if n == 1 { return 1; }
    let mut count_of_numbers = 0;

    for i in 1..=((n as f32).sqrt() as usize) {
        if n % i as u32 != 0 { continue; }
        if (i * i) as u32 == n { count_of_numbers += 1; } else { count_of_numbers += 2; }
    }
    count_of_numbers
}

#[cfg(test)]
mod tests {
    use super::divisors;

    #[test]
    fn sample_tests() {
        assert_eq!(divisors(1), 1);
        assert_eq!(divisors(4), 3);
        assert_eq!(divisors(5), 2);
        assert_eq!(divisors(12), 6);
        assert_eq!(divisors(25), 3);
        assert_eq!(divisors(4096), 13);
    }
}

/*
Count the number of divisors of a positive integer n.

Random tests go up to n = 500000.
Examples (input --> output)

4 --> 3 (1, 2, 4)
5 --> 2 (1, 5)
12 --> 6 (1, 2, 3, 4, 6, 12)
30 --> 8 (1, 2, 3, 5, 6, 10, 15, 30)

Note you should only return a number, the count of divisors. The numbers between parentheses are shown only for you to see which numbers are counted in each case.

*/