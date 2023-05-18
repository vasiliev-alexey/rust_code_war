use regex::Regex;

fn order(sentence: &str) -> String {

    let  mut words =   sentence.split_whitespace().collect::<Vec<_>>();
    let re = Regex::new(r"\d").unwrap();
    words.sort_by( |x ,  y| {
      let x_pos =   re.find_iter(x).last().unwrap().as_str();
      let y_pos =   re.find_iter(y).last().unwrap().as_str();
        return  x_pos.cmp(y_pos)
    } );
    return  words.join(" ")

}
//https://www.codewars.com/kata/55c45be3b2079eccff00010f/train/rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn returns_expected() {
        assert_eq!(order("is2 Thi1s T4est 3a"), "Thi1s is2 3a T4est");
        assert_eq!(order(""), "");
    }
}
