use std::collections::HashMap;


fn count_duplicates(text: &str) -> u32 {
    // Your code goes here
    let mut char_counts = HashMap::new();
    for x in text.to_lowercase().chars() {
        *char_counts.entry(x).or_insert(0) += 1;

    }
     let mut rez = 0;
    for (_ , cnt) in char_counts.iter() {
        if *cnt > 1 {
            rez +=1;
        }
    }
    println!("{char_counts:?}");
    rez
}

//https://www.codewars.com/kata/54bf1c2cd5b56cc47f0007a1/train/rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_abcde() {
        assert_eq!(count_duplicates("abcde"), 0);
    }

    #[test]
    fn test_abcdea() {
        assert_eq!(count_duplicates("abcdea"), 1);
    }

    #[test]
    fn test_indivisibility() {
        assert_eq!(count_duplicates("indivisibility"), 1);
    }    #[test]
    fn test_qqq() {
        assert_eq!(count_duplicates("aabBcde"), 2);
    }
}
