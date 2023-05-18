fn is_prime(x: u32) -> bool {
    if x <= 1 {
        return false;
    }


    for i in 2..((x as f64).sqrt() as u32 + 1) {
        if x % i == 0 {
            return false;
        }
    }
    return true;
}

fn minimum_number(xs: &[u32]) -> u32 {
    let mut sum = xs.iter().sum::<u32>();
    let mut rez = 0u32;
    while !is_prime(sum) {
        sum += 1;
        rez += 1;
    }
    rez
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn basic() {
        // assert_eq!(minimum_number(&[3,1,2]), 1);
        // assert_eq!(minimum_number(&[5,2]), 0);
        // assert_eq!(minimum_number(&[1,1,1]), 0);
        assert_eq!(minimum_number(&[2, 12, 8, 4, 6]), 5);
        assert_eq!(minimum_number(&[50, 39, 49, 6, 17, 28]), 2);
    }
}