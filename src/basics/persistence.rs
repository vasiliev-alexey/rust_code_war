use itertools::Itertools;

fn persistence(num: u64) -> u64 {
    if num < 10 {
        return 0;
    }
    let mut n = num;
    let mut i = 0u8;

    while n >= 10 {
        i = i + 1;
        let mut num_chars = n.to_string().chars().collect_vec();
        n = 1;
        for x in num_chars {
            n = n * x.to_digit(10).unwrap() as u64;
        }
      }
    i as u64
}


// https://www.codewars.com/kata/55bf01e5a717a0d57e0000ec/train/rust

#[cfg(test)]
mod tests {
    #[test]
    fn sample_tests() {
        assert_eq!(super::persistence(39), 3);
        assert_eq!(super::persistence(4), 0);
        assert_eq!(super::persistence(25), 2);
        assert_eq!(super::persistence(999), 4);
    }
}