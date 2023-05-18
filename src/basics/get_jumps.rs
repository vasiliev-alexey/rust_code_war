fn get_jumps(cycle: Vec<i32>, k: i32) -> i32 {
    let mut cnt : i64 = 1;
    let t = k.clone() as i64;
    let mut i : i64= 0;
    while (i + t ) % cycle.len() as i64 > 0 {
        cnt = cnt + 1;
        i = i + t;
    }
    return cnt as i32;
}


mod tests {
    use super::*;

    #[test]
    fn test_fixed() {
        assert_eq!(get_jumps(vec![1, 5, 7, 8, 9], 1), 5);
        assert_eq!(get_jumps(vec![1, 5, 7, 8, 9], 2), 5);
        assert_eq!(get_jumps(vec![1, 5, 1], 2), 3);
    }
}