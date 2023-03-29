fn find_multiples(n: u32, limit: u32) -> Vec<u32> {
    let mut rez = vec![];
    for i in 1.. limit/n +1  {
           rez.push( i * n );
    }
    rez
}



// https://www.codewars.com/kata/58ca658cc0d6401f2700045f/train/rust
#[cfg(test)]
mod tests {
    use super::find_multiples;

    #[test]
    fn basic_test() {
        assert_eq!(find_multiples(1, 2), [1, 2]);
        assert_eq!(find_multiples(5, 7), [5]);
        assert_eq!(find_multiples(4, 27), [4, 8, 12, 16, 20, 24]);
        assert_eq!(find_multiples(11, 54), [11, 22, 33, 44]);
        assert_eq!(find_multiples(5, 25), [5, 10, 15, 20, 25]);
    }
}

// https://www.codewars.com/kata/557cd6882bfa3c8a9f0000c1/train/rust
fn get_age(age: &str) -> u32 {
    age.chars().next().unwrap().to_digit(10).unwrap()
}

#[test]
fn basic_tests() {
    assert_eq!(get_age("2 years old"), 2);
    assert_eq!(get_age("4 years old"), 4);
    assert_eq!(get_age("5 years old"), 5);
    assert_eq!(get_age("7 years old"), 7);
}