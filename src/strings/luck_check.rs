use itertools::Itertools;

fn luck_check(ticket: &str) -> Option<bool> {


   let chars = ticket.chars().collect_vec();
    if chars.iter().filter(  |c| !c.is_digit(10)).collect_vec().len() > 0  || ticket.len() < 2{
        return  None
    }
    let middle = if chars.len()%2 ==0 {
        chars.len()/2
    } else {
        chars.len()/2
    };
    let left =  &chars[0..middle].iter().map( |c| c.to_digit(10).unwrap()).fold( 0 , |x, y| x+y);
    let right =&chars.iter().rev().collect_vec()[0 .. middle].iter().map( |c| c.to_digit(10).unwrap()).fold( 0 , |x, y| x+y);
    println!("{} {right} {:?} {:?}", left , &chars[0..middle], &chars.iter().rev().collect_vec()[0 .. middle]);

    Some(left == right)
}

// Add your tests here.
// See https://doc.rust-lang.org/stable/rust-by-example/testing/unit_testing.html

#[cfg(test)]
mod tests {
    use super::luck_check;

    fn dotest(s: &str, expected: Option<bool>) {
        let actual = luck_check(s);
        assert!(actual == expected, "With ticket = \"{s}\"\nExpected {expected:?} but got {actual:?}")
    }

    #[test]
    fn fixed_tests() {
        dotest("91856399083", Some(true));
        dotest("6F43E8", None);
        dotest("683179", Some(true));
        dotest("683000", Some(false));
        //

    }
}
