use regex::Regex;

fn solve(s: &str) -> u32 {


   println!("{:?}" ,  Regex::new(r"\d+").unwrap().find_iter(s).map(  |e| e.as_str().parse::<i32>().unwrap()).max());


    Regex::new(r"\d+").unwrap().find_iter(s).map(  |e| e.as_str().parse::<u32>().unwrap()).max().unwrap()
}

#[cfg(test)]
mod tests {
    use super::solve;

    #[test]
    fn basic_tests() {
        assert_eq!(solve(&"gh12cdy695m1"), 695);
        assert_eq!(solve(&"2ti9iei7qhr5"), 9);
        assert_eq!(solve(&"lu1j8qbbb85"), 85);
        assert_eq!(solve(&"185lu1j8qbbb85"), 185);
    }
}

/* Numbers in strings

https://www.codewars.com/kata/59dd2c38f703c4ae5e000014/train/rust
In this Kata, you will be given a string that has lowercase letters and numbers. Your task is to compare the number groupings and return the largest number. Numbers will not have leading zeros.

For example, solve("gh12cdy695m1") = 695, because this is the largest of all number groupings.

Good luck!
*/