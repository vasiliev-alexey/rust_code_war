use itertools::Itertools;

fn special_number(n: u64) -> String {
    let res = n.to_string().chars().map(|x| x.to_digit(10).unwrap() as u8).filter(|x | *x >= 6).collect_vec().len() > 0;
    return if res { "NOT!!".to_string() } else { "Special!!".to_string() };
}
//https://www.codewars.com/kata/5a55f04be6be383a50000187/train/rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn basic() {
        assert_eq!(special_number(2),"Special!!");
        assert_eq!(special_number(3),"Special!!");
        assert_eq!(special_number(6),"NOT!!");
        assert_eq!(special_number(9),"NOT!!");
        assert_eq!(special_number(11),"Special!!");
        assert_eq!(special_number(55),"Special!!");
        assert_eq!(special_number(26),"NOT!!");
        assert_eq!(special_number(92),"NOT!!");
        assert_eq!(special_number(25432),"Special!!");
        assert_eq!(special_number(2783),"NOT!!");
    }
}
