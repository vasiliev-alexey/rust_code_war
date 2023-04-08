

fn factorial(num: u64) -> u64 {
    match num {
        0 | 1 => 1,
        _ => factorial(num - 1) * num,
    }

}
//  (1..=x).into_iter().product()

fn strong(n: u64) -> String {
    if   n.to_string().chars().map( |c| c.to_digit(10).unwrap()).fold( 0u64, |x, y|  x + factorial(y as u64))  == n {
        return  "STRONG!!!!".to_string();
    }
    return  "Not Strong !!".to_string();
}

// Add your tests here.
// See https://doc.rust-lang.org/stable/rust-by-example/testing/unit_testing.html

//https://www.codewars.com/kata/5a4d303f880385399b000001/train/rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn basic() {
        // Testing for 1
        assert_eq!(strong(1), "STRONG!!!!");

        // Testing for 2
        assert_eq!(strong(2), "STRONG!!!!");

        // Testing for 145
        assert_eq!(strong(145), "STRONG!!!!");

        // Testing for 7
        assert_eq!(strong(7), "Not Strong !!");

        // Testing for 93
        assert_eq!(strong(93), "Not Strong !!");

        // Testing for 185
        assert_eq!(strong(185), "Not Strong !!");
    }
}
