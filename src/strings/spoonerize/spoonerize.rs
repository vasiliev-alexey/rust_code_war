fn spoonerize(words: &str) -> String {
    let word1 = words.split_whitespace().next().unwrap();
    let word2 = words.split_whitespace().nth(1).unwrap();
    let a = word1.chars().next().unwrap();
    let b = word2.chars().next().unwrap();


    format!("{} {}", word1.chars().enumerate().map(|(i, c)| if i == 0 { b } else { c }).collect::<String>(), word2.chars().enumerate().map(|(i, c)| if i == 0 { a } else { c }).collect::<String>())
}
/*
  let mut w: Vec<_> = words.chars().collect();
    let mut i = 0; while w[i] !=' ' {i+=1};
    w.swap(0, i+1);
    w.into_iter().collect()
*/

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn basic() {
        assert_eq!(spoonerize("nit picking"), "pit nicking");
        assert_eq!(spoonerize("wedding bells"), "bedding wells");
        assert_eq!(spoonerize("jelly beans"), "belly jeans");
    }
}
