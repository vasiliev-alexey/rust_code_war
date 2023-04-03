
fn are_you_playing_banjo(name: &str) -> String {

    if name.chars().next().unwrap().to_uppercase().to_string() == "R" {
        return  format!("{name} plays banjo");
    }
   return  format!("{name} does not play banjo");
}

// Add your tests here.
// See https://doc.rust-lang.org/stable/rust-by-example/testing/unit_testing.html
//https://www.codewars.com/kata/53af2b8861023f1d88000832/train/rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_are_you_playing_banjo() {
        assert_eq!(are_you_playing_banjo("Martin"), "Martin does not play banjo");
        assert_eq!(are_you_playing_banjo("Rikke"), "Rikke plays banjo");
        assert_eq!(are_you_playing_banjo("bravo"), "bravo does not play banjo");
        assert_eq!(are_you_playing_banjo("rolf"), "rolf plays banjo");
    }
}
