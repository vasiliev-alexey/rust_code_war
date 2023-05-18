use itertools::Itertools;

fn name_shuffler(s: &str) -> String {
   let s =  s.split_whitespace().collect_vec();
    format!("{} {}" ,  s.get(1).unwrap()  ,  s.get(0).unwrap())
}
// https://www.codewars.com/kata/559ac78160f0be07c200005a/train/rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_basic() {
        assert_eq!(name_shuffler("john McClane"), "McClane john");
        assert_eq!(name_shuffler("Mary jeggins"), "jeggins Mary");
        assert_eq!(name_shuffler("tom jerry"), "jerry tom");
    }
}