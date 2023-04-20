fn words_to_marks(s: &str) -> u32 {

   return  s.chars().map( |x| x as u32 -96 ).fold(0 , |a, b| a+b)  ;

}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn basic() {
        assert_eq!(words_to_marks("attitude"), 100);
        assert_eq!(words_to_marks("friends"), 75);
        assert_eq!(words_to_marks("family"), 66);
        assert_eq!(words_to_marks("selfness"), 99);
        assert_eq!(words_to_marks("knowledge"), 96);
    }
}