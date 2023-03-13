fn digital_root(n: i64) -> i64 {

    let mut str = n.to_string();
    while str.len() > 1 {
        str = str.split("").map(|x| x.parse::<u8>().unwrap_or(0)).fold(0 as u8, |x, y| x + y).to_string();
    }
    str.parse::<i64>().unwrap()
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn returns_expected() {
        assert_eq!(digital_root(16), 7);
        assert_eq!(digital_root(456), 6);
        assert_eq!(digital_root(132189), 6);
        assert_eq!(digital_root(493193), 2);



    }
}

/*
https://www.codewars.com/kata/541c8630095125aba6000c00/train/rust
Output
Past Solutions

Digital root is the recursive sum of all the digits in a number.

Given n, take the sum of the digits of n. If that value has more than one digit, continue reducing in this way until a single-digit number is produced. The input will be a non-negative integer.
Examples

    16  -->  1 + 6 = 7
   942  -->  9 + 4 + 2 = 15  -->  1 + 5 = 6
132189  -->  1 + 3 + 2 + 1 + 8 + 9 = 24  -->  2 + 4 = 6
493193  -->  4 + 9 + 3 + 1 + 9 + 3 = 29  -->  2 + 9 = 11  -->  1 + 1 = 2

*/