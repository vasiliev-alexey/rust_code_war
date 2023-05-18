fn choose_best_sum(t: i32, k: i32, ls: &Vec<i32>) -> i32 {
    let lst = ls;
    let mut i = 0;
    let mut d = 0;
    while (i + k - 1) < lst.len() as i32
    {
        let mut dis = ls[i as usize];
        if k > 1
        {
            let save_one = lst[i as usize];
            dis = choose_best_sum(t - save_one, k - 1, &(lst[((i + 1) as usize)..]).to_vec());
            if dis > 0
            {
                dis = save_one + dis;
            } else {
                dis = -1;
            }
        }
        if t - dis < t - d && t - dis >= 0
        {
            d = dis;
        }
        i += 1;
    }
    if d <= 0
    {
        return -1;
    } else {
        return d;
    }
}

fn testing(t: i32, k: i32, ls: &Vec<i32>, exp: i32) -> () {
    assert_eq!(choose_best_sum(t, k, ls), exp)
}

// https://www.codewars.com/kata/55e7280b40e1c4a06d0000aa/train/rust
#[test]
fn basics_choose_best_sum() {
    let ts = &vec![50, 55, 56, 57, 58];
    testing(163, 3, ts, 163);
    let ts = &vec![50];
    testing(163, 3, ts, -1);
    let ts = &vec![91, 74, 73, 85, 73, 81, 87];
    testing(230, 3, ts, 228);
    testing(331, 2, ts, 178);
}