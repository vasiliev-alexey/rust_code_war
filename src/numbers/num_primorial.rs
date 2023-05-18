fn is_prime(x: u64) -> bool {
    if x <= 1 {
        return false;   }


    for i in 2 .. (( x as f64).sqrt() as i64 + 1)  {
        if x % i as u64 == 0u64 {
            return false;
        }
    }
    return true;

}

fn num_primorial(n: usize) -> u64 {
    let mut  rez = 1u64;
    let mut i = 1;
    let mut prime_cnt = 0;

    while  prime_cnt < n {
            if is_prime(i as u64) {
                rez = rez * i as u64;
                prime_cnt = 1 + prime_cnt;
            }
        i+=1;
    }
    rez
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_basic() {
        assert_eq!(num_primorial(3), 30);
        assert_eq!(num_primorial(4), 210);
        assert_eq!(num_primorial(5), 2310);
        assert_eq!(num_primorial(8), 9699690);
        assert_eq!(num_primorial(9), 223092870);
    }
}
