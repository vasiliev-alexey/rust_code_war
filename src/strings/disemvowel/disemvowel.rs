use itertools::Itertools;

fn disemvowel(s: &str) -> String {
    s.chars().filter( |x| !['a', 'e', 'i', 'o', 'u' ].contains(&x.to_ascii_lowercase() )).join("")
}


#[cfg(test)]
mod tests {
    use super::disemvowel;

    #[test]
    fn example_test() {
        assert_eq!(disemvowel("This website is for losers LOL!"), "Ths wbst s fr lsrs LL!");
    }
}