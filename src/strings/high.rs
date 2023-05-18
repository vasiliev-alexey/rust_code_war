fn high(input: &str) -> &str {
    let words = input.split_whitespace().collect::<Vec<_>>();
    let mut swap = 0;
    let mut rez = "";
    for x in words {
        let cost = x.to_string().chars().fold(0 as u32, |x, c|
            x + "abcdefghijklmnopqrstuvwxyz".find(c).unwrap() as u32 + 1,
        );

        if cost > swap {
            swap = cost;
            rez = x;
        }
    }
    return rez;
 }

#[cfg(test)]
mod tests {
    use super::high;

    #[test]
    fn test_basic() {
        //  assert_eq!(high("man i need a taxi up to ubud"), "taxi");

        assert_eq!(high("what time are we climbing up the volcano"), "volcano");
        assert_eq!(high("take me to semynak"), "semynak");
        assert_eq!(high("massage yes massage yes massage"), "massage");
        assert_eq!(high("take two bintang and a dance please"), "bintang");
        assert_eq!(high("aa b"), "aa");
        assert_eq!(high("b aa"), "b");
        assert_eq!(high("bb d"), "bb");
        assert_eq!(high("d bb"), "d");
        assert_eq!(high("aaa b"), "aaa");
    }
}

/*

Highest Scoring Word

Given a string of words, you need to find the highest scoring word.
Each letter of a word scores points according to its position in the alphabet: a = 1, b = 2, c = 3 etc.
For example, the score of abad is 8 (1 + 2 + 1 + 4).
You need to return the highest scoring word as a string.
If two words score the same, return the word that appears earliest in the original string.
All letters will be lowercase and all inputs will be valid.
*/