fn longest_repetition(s: &str) -> Option<(char, usize)> {
    if s.len() == 0 {
        return None;
    }
    let mut prev = vec![];
    let mut rez = vec![];
    let mut prev_char = s.chars().next().unwrap();
    for x in s.chars() {
        if prev_char == x {
            prev.push(x)
        } else {
            rez.push(prev.clone());
            prev.clear();
            prev.push(x);
            prev_char = x;
        }
    }
    rez.push(prev.clone());

    let mut winner: char = '\0';
    let mut tmp = 0;
    for x in rez {
        println!("{:?}", x);

        if x.len() > tmp {
            winner = *x.get(0).unwrap();
            tmp = x.len().clone();
        }
    }
    return Option::from((winner, tmp));
}

#[cfg(test)]
mod tests {
    use super::longest_repetition;

    #[test]
    fn longest_at_the_beginning() {
        assert_eq!(longest_repetition(&"aaaabbb"), Some(('a', 4)));
    }

    #[test]
    fn longest_at_the_end() {
        assert_eq!(longest_repetition(&"abbbbb"), Some(('b', 5)));
      assert_eq!(longest_repetition(&"bbbaaabaaaa"), Some(('a', 4)));
    }

    #[test]
    fn longest_in_the_middle() {
        assert_eq!(longest_repetition(&"cbdeuuu900"), Some(('u', 3)));
    }

    #[test]
    fn multiple_longest() {
        assert_eq!(longest_repetition(&"aabb"), Some(('a', 2)));
        assert_eq!(longest_repetition(&"ba"), Some(('b', 1)));
    }

    #[test]
    fn single_character_only() {
        assert_eq!(longest_repetition(&"aaaaaa"), Some(('a', 6)));
    }

    #[test]
    fn empty_string() {
        assert_eq!(longest_repetition(&""), None);
    }
}
/*
Character with longest consecutive repetition

For a given string s find the character c (or C) with longest consecutive repetition and return:

Some((c, l))
where l (or L) is the length of the repetition. If there are two or more characters with the same l return the first in order of appearance.

For empty string return:

None
Happy coding! :)
https://www.codewars.com/kata/586d6cefbcc21eed7a001155/train/rust
*/

