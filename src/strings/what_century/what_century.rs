fn what_century(year: &str) -> String {
    let num_year = 1 + (year.parse::<i32>().unwrap() - 1) / 100;
    let mut result = String::new();
    result.push_str(&*num_year.to_string());
    match num_year {
        11 | 12 | 13 => result.push_str("th"),
        _ if num_year % 10 == 1 => result.push_str("st"),
        _ if num_year % 10 == 2 => result.push_str("nd"),
        _ if num_year % 10 == 3 => result.push_str("rd"),
        _ => result.push_str("th"),
    }
    result

    /*
        let century = (year.parse::<u32>().unwrap() + 99) / 100;
    let suffix = if century < 20 {
        "th"
    } else {
        match century % 10 {
            1 => "st",
            2 => "nd",
            3 => "rd",
            _ => "th",
        }
    };

    format!("{}{}", century, suffix)
    */

}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_basic() {
        assert_eq!(what_century("1999"), "20th");
        assert_eq!(what_century("2011"), "21st");
        assert_eq!(what_century("2154"), "22nd");
        assert_eq!(what_century("2259"), "23rd");
        assert_eq!(what_century("1234"), "13th");
        assert_eq!(what_century("1023"), "11th");
        assert_eq!(what_century("2000"), "20th");
        assert_eq!(what_century("3210"), "33rd");
    }
}
