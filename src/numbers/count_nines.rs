fn count_nines(n: u64) -> u64 {
    let mut tmp_n = n;
    let mut num = 0;
    let mut l = n.to_string().len() as u64;

    while l > 0 {
        let k = 10u64.pow(l as u32);
        let m = 10u64.pow((l - 1) as u32);
        num += (tmp_n / k) * (l * m);
        tmp_n = tmp_n % k;
        if tmp_n >= k - m {
            num += tmp_n % m + 1;
        }

        l = l - 1;
    }
    num
}

#[cfg(test)]
mod tests {
    use super::count_nines;

    const ERR_MSG: &str = "\nYour result (left) did not match the expected output (right)";

    #[test]
    fn basic() {
        assert_eq!(count_nines(1), 0, "Nein!{ERR_MSG} with n = 1");
        assert_eq!(count_nines(9), 1, "Nein!{ERR_MSG} with n = 9");
        assert_eq!(count_nines(100), 20, "Nein!{ERR_MSG} with n = 100");
        assert_eq!(count_nines(565_754), 275_645, "Nein!{ERR_MSG} with n = 565,754");
        assert_eq!(count_nines(10_000_000_000), 10_000_000_000, "Nein!{ERR_MSG} with n = 10,000,000,000");
    }
}

/*
https://www.codewars.com/kata/55143152820d22cdf00001bb/train/rust
It's 9 time!

I want to count from 1 to n. How many times will I use a '9'?

9, 19, 91.. will contribute one '9' each, 99, 199, 919.. wil contribute two '9's each...etc

Note: n will always be a positive integer.

Examples (input -> output)
8  -> 0
9  -> 1
10 -> 1
98 -> 18
100 -> 20

*/