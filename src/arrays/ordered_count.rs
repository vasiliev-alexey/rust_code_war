use std::collections::HashMap;


fn ordered_count(sip: &str) -> Vec<(char, i32)> {
    let mut hs = HashMap::new();
    let mut keys = Vec::new();
    let mut vec: Vec<(char, i32)> = Vec::new();


    for c in sip.chars() {
        if !hs.contains_key(&c) {
            keys.push(c)
        }
        *hs.entry(c).or_insert(0) += 1;
    }

    for c in keys.iter() {
        vec.push((*c, hs[c]))
    }

    vec
    //sip.chars().unique().map(|c| (c, sip.matches(c).count() as i32)).collect()
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_abradacadabra() {
        assert_eq!(
            ordered_count("abracadabra"),
            vec![('a', 5), ('b', 2), ('r', 2), ('c', 1), ('d', 1)]
        );
    }

    #[test]
    fn test_banana() {
        assert_eq!(ordered_count("banana"), vec![('b', 1), ('a', 3), ('n', 2)]);
    }

    #[test]
    fn test_master_solver() {
        assert_eq!(
            ordered_count("i am a master kata solver"),
            vec![
                ('i', 1),
                (' ', 5),
                ('a', 5),
                ('m', 2),
                ('s', 2),
                ('t', 2),
                ('e', 2),
                ('r', 2),
                ('k', 1),
                ('o', 1),
                ('l', 1),
                ('v', 1),
            ]
        );
    }
}

/*
Count the number of occurrences of each character and return it as a (list of tuples) in order of appearance. For empty output return (an empty list).
Consult the solution set-up for the exact data structure implementation depending on your language.

Example:
```rust
ordered_count("abracadabra") == vec![('a', 5), ('b', 2), ('r', 2), ('c', 1), ('d', 1)]
```
*/