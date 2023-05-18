fn count_sheep(n: u32) -> String {
    if n ==0 {
        return  "".to_string();
    }
    let mut vec = Vec::with_capacity(n as usize);
    for i in 0..n {
        vec.push(format!("{} sheep..." , i+1));
    }
    return   vec.iter().join("")
}
// https://www.codewars.com/kata/5b077ebdaf15be5c7f000077/train/rust
#[cfg(test)]
extern crate rand;

use itertools::Itertools;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn returns_expected() {
        assert_eq!(count_sheep(0), "");
        assert_eq!(count_sheep(1), "1 sheep...");
        assert_eq!(count_sheep(2), "1 sheep...2 sheep...");
        assert_eq!(count_sheep(3), "1 sheep...2 sheep...3 sheep...");
    }
}