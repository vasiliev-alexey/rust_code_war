fn fortune(f0: i32, p: f64, c0: i32, n: i32, i: f64) -> bool {
    if f0 < 0 {
        return false;
    }
    if n == 1 {
        return true;
    }
    let mut deposit = f0;
    let inf_coeff = 1.0 + i / 100.0;

    for j in 1..n {
        let withdraw = (c0 as f64 * inf_coeff.powi(j - 1)) as i32;
        deposit = deposit + (deposit as f64 * p / 100.0) as i32 - withdraw;
    }
    return deposit >= 0;
}

// https://www.codewars.com/kata/56445c4755d0e45b8c00010a/train/rust
#[cfg(test)]
mod tests {
    use super::fortune;

    #[test]
    fn basics() {
        assert_eq!(fortune(100_000, 1.0, 2_000, 15, 1.0), true);
        assert_eq!(fortune(100_000, 1.0, 9_185, 12, 1.0), false);
        assert_eq!(fortune(100_000_000, 1.0, 100_000, 50, 1.0), true);
        assert_eq!(fortune(100_000_000, 1.5, 10_000_000, 50, 1.0), false);
    }
}