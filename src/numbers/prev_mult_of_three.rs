fn prev_mult_of_three(n: i32) -> i32 {

    for x in n.to_string().chars().enumerate() {
        let st =  n / 10_i32.pow(x.0 as u32);
        let res = st%3;
        println!("{st}");
        if (res % 3) ==0 {
            return  st  as i32
        }

    }




    -1_i32
}

// Add your tests here.
// See https://doc.rust-lang.org/stable/rust-by-example/testing/unit_testing.html

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn basic_tests() {
        assert_eq!(prev_mult_of_three(1), -1);
        assert_eq!(prev_mult_of_three(25), -1);
        assert_eq!(prev_mult_of_three(36), 36);
        assert_eq!(prev_mult_of_three(1244), 12);
        assert_eq!(prev_mult_of_three(952406), 9);
    }
}

