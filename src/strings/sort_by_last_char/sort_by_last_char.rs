

fn sort_by_last_char(s: &str) -> Vec<String> {
//use itertools::Itertools;
    // let mut s = s
    //     .split(" ")
    //     .map(|w| (w, w.chars().last().unwrap()))
    //     .collect_vec();
    // s.sort_by(|a, b| a.1.cmp(&b.1));
    // s.into_iter().map(|item| item.0.to_string()).collect_vec()
    //



    let mut v = s.split(' ').map(|s| s.to_string()).collect::<Vec<_>>();
    v.sort_by_key(|k| k.chars().last());
    v
}


#[cfg(test)]
mod tests {
    use super::sort_by_last_char;

    #[test]
    fn sample_tests() {
        assert_eq!(sort_by_last_char("man i need a taxi up to ubud"),
                   vec!["a", "need", "ubud", "i", "taxi", "man", "to", "up"]);
        assert_eq!(sort_by_last_char("what time are we climbing up the volcano"),
                   vec!["time", "are", "we", "the", "climbing", "volcano", "up", "what"]);
        assert_eq!(sort_by_last_char("take me to semynak"),
                   vec!["take", "me", "semynak", "to"]);
        assert_eq!(sort_by_last_char("massage yes massage yes massage"),
                   vec!["massage", "massage", "massage", "yes", "yes"]);
        assert_eq!(sort_by_last_char("take bintang and a dance please"),
                   vec!["a", "and", "take", "dance", "please", "bintang"]);
    }
}
