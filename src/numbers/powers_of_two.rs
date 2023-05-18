fn powers_of_two(n: u8) -> Vec<u128> {
    let mut v: Vec<u128> =   Vec::new();
    for index in  0..n+1 {
           v.push((2 as u8).pow(index as u32) as u128)
    }
    v
}

//  (0..=n).map(|x| 2_u128.pow(x as u32)).collect()

#[cfg(test)]
mod tests {
    use super::powers_of_two;

    fn dotest(n: u8, expected: &[u128]) {
        let actual = powers_of_two(n);
        assert!(actual == expected, "With n = {n}\nExpected {expected:?}\nBut got {actual:?}")
    }

    #[test]
    fn fixed_tests() {
       dotest(0, &[1]);
        dotest(1, &[1, 2]);
        dotest(4, &[1, 2, 4, 8, 16]);
    }
}

/*
Complete the function that takes a non-negative integer n as input, and returns a list of all the powers of 2 with the exponent ranging from 0 to n ( inclusive ).
Examples

n = 0  ==> [1]        # [2^0]
n = 1  ==> [1, 2]     # [2^0, 2^1]
n = 2  ==> [1, 2, 4]  # [2^0, 2^1, 2^2]
*/