
fn reverse_words_2(words: &str) -> String {
   words.split_whitespace().map( |x|  x.to_string()).rev().collect::<Vec<String>>().join( " ")

}


#[cfg(test)]
mod tests {
    use super::reverse_words_2;

    #[test]
    fn returns_expected() {
        assert_eq!(reverse_words_2("hello world!"), "world! hello");
    }
}