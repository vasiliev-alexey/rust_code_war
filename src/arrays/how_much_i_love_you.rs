fn how_much_i_love_you(nb_petals: u16) -> &'static str {

    match nb_petals % 6 {
        1 => "I love you",
        2 => "a little",
        3 =>"a lot",
        4 =>"passionately",
        5 =>"madly",
        _ =>"not at all",

    }

}


// Add your tests here.
// See https://doc.rust-lang.org/stable/rust-by-example/testing/unit_testing.html
//https://www.codewars.com/kata/57f24e6a18e9fad8eb000296/train/rust
#[cfg(test)]
mod tests {
    use super::how_much_i_love_you;

    #[test]
    fn fixed_tests() {
        assert_eq!(how_much_i_love_you(7), "I love you");
        assert_eq!(how_much_i_love_you(3), "a lot");
        assert_eq!(how_much_i_love_you(6), "not at all");
    }
}
