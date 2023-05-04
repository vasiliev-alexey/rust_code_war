

fn square_digits(num: u64) -> u64 {
    let numbs =  num.to_string().chars().map(|c| c.to_digit(10).unwrap()).map( |x| x.pow(2).to_string()).collect::<Vec<_>>().join("").parse::<u64>().unwrap();
    // println!("{:?}", numbs.parse::<u64>().unwrap());
    numbs

}

// https://doc.rust-lang.org/stable/rust-by-example/testing/unit_testing.html

#[cfg(test)]
mod tests {
    use super::square_digits;

    #[test]
    fn test_square_digits() {
        assert_eq!(square_digits(9119), 811181, "\nFailed with num 9119");
    }
}
