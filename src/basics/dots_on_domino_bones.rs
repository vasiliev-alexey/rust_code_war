fn dots_on_domino_bones(n: i32) -> Option<u64> {
  let mut res =  0_u64;
    for i in 0_u64.. (n+1) as u64 {
        for j in i..(n +1) as u64 {
         if j==0 {
             continue
         }
           // println!("{} {}" , i , j);
           res = res  + i + j;
        }
    }
    Some(res)

}

// Add your tests here.
// See https://doc.rust-lang.org/stable/rust-by-example/testing/unit_testing.html

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_dots_on_domino_bones() {
        assert_eq!(dots_on_domino_bones(2), Some(12));
        assert_eq!(dots_on_domino_bones(5), Some(105));
        assert_eq!(dots_on_domino_bones(13), Some(1365));
        assert_eq!(dots_on_domino_bones(20), Some(4620));
        assert_eq!(dots_on_domino_bones(33), Some(19635));
        assert_eq!(dots_on_domino_bones(50), Some(66300));
        assert_eq!(dots_on_domino_bones(0), Some(0));
        assert_eq!(dots_on_domino_bones(137), Some(1313967));
        assert_eq!(dots_on_domino_bones(198), Some(3940200));
        assert_eq!(dots_on_domino_bones(2267), Some(5833095282));
    }
}
