fn add_length(s: &str) -> Vec<String> {
   s.split_whitespace().map(|s| format!("{} {}", s, s.len())).collect::<Vec<_>>()

}

#[cfg(test)]
mod tests {
    use super::add_length;

    fn dotest(s: &str, expected: &[String]) {
        let actual = add_length(s);
        assert!(actual == expected,
                "With s = \"{s}\"\nExpected {expected:?} but got {actual:?}")
    }

    #[test]
    fn fixed_tests() {
        dotest("apple ban",&["apple 5", "ban 3"].iter().map(|x| x.to_string()).collect::<Vec<_>>());
        dotest("you will win",&["you 3", "will 4", "win 3"].iter().map(|x| x.to_string()).collect::<Vec<_>>());
        dotest("y",&["y 1"].iter().map(|x| x.to_string()).collect::<Vec<_>>());
    }
}


/*
What if we need the length of the words separated by a space to be added at the end of that same word and have it returned as an array?
Example(Input --> Output)
"apple ban" --> ["apple 5", "ban 3"]
"you will win" -->["you 3", "will 4", "win 3"]
Your task is to write a function that takes a String and returns an Array/list with the length of each word added to each element .
Note: String will have at leas
*/