use itertools::Itertools;

fn solve(a: &str, b: &str) -> String {
    let c1 = a.chars().filter(|c| !b.chars().contains(&c)).collect_vec();
    let c2 = b.chars().filter(|c| !a.chars().contains(&c)).collect_vec();
    return c1.iter().join("") + &c2.iter().join("");
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_basics() {
        assert_eq!(solve("xyab", "xzca"), "ybzc".to_string());
        assert_eq!(solve("xyabb", "xzca"), "ybbzc".to_string());
        assert_eq!(solve("abcd", "xyz"), "abcdxyz".to_string());
        assert_eq!(solve("xxx", "xzca"), "zca".to_string());
    }
}
