fn remove_nb(m: i32) -> Vec<(i32, i32)> {

    let num: i64 = m as i64;
    let total: i64 = num * (num + 1) / 2;
    let mut res = vec![];
    for a in 1..num {
        if (total - a) % (a + 1) == 0 {
            let b = (total - a) / (a + 1);
            if b < num {
                res.push((a as i32, b as i32))
            }
        }
    }
    res

}

// https://www.codewars.com/kata/5547cc7dcad755e480000004/train/rust
fn testing(n: i32, exp: Vec<(i32, i32)>) -> () {
    assert_eq!(remove_nb(n), exp)
}

#[test]
fn basics_remove_nb() {
    testing(26, vec![(15, 21), (21, 15)]);
    testing(100, vec![]);
    testing(101, vec![(55, 91), (91, 55)]);
    testing(102, vec![(70, 73), (73, 70)]);
    // testing(1000003, vec![(70, 73), (73, 70)]);
}