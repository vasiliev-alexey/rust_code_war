

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

/*
Welcome. In this kata, you are asked to square every digit of a number and concatenate them.
For example, if we run 9119 through the function, 811181 will come out, because 92 is 81 and 12 is 1. (81-1-1-81)
Example #2: An input of 765 will/should return 493625 because 72 is 49, 62 is 36, and 52 is 25. (49-36-35)
Note: The function accepts an integer and returns an integer.
Happy Coding!
*/