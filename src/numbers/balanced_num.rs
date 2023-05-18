fn balanced_num(n: u64) -> String {

    let  nums = &n.to_string();
     let mut   n_len =  nums.len();
    if n_len< 3 {
        return  "Balanced".to_string();
    }

    if n_len%2 == 0 {
        n_len = n_len -1;
    }

    let left = nums.chars().take(n_len/2).map(  |e| e.to_digit(10).unwrap()).fold( 0 , |x, y|  x+y);
    let right = nums.chars().rev().take(n_len/2).map(  |e| e.to_digit(10).unwrap()).fold( 0 , |x, y|  x+y);
    if left == right {
        return  "Balanced".to_string();
    }
    "Not Balanced".to_string()
}
// https://www.codewars.com/kata/5a4e3782880385ba68000018/train/rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn balanced_number() {
        assert_eq!(balanced_num(7), "Balanced");
        assert_eq!(balanced_num(959), "Balanced");
        assert_eq!(balanced_num(13), "Balanced");
        assert_eq!(balanced_num(432), "Not Balanced");
        assert_eq!(balanced_num(424), "Balanced");
    }

    #[test]
    fn larger_number() {
        assert_eq!(balanced_num(1024), "Not Balanced");
        assert_eq!(balanced_num(66545), "Not Balanced");
        assert_eq!(balanced_num(295591), "Not Balanced");
        assert_eq!(balanced_num(1230987), "Not Balanced");
        assert_eq!(balanced_num(56239814), "Balanced");
    }
}