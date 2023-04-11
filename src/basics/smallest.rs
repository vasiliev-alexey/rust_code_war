use itertools::Itertools;

fn smallest(n: i64) -> (i64, usize, usize) {
    let nums = n.to_string().chars().map(|c| c.to_digit(10).unwrap()).collect_vec();
    let mut rez = vec![];
    for i in 0..nums.len() {
        let cur_num = nums[i];
        let rest_nums = nums.iter().enumerate().filter(|(ind, _)| i != *ind).map(|v| *v.1).collect::<Vec<u32>>();
        for j in 0..nums.len() {
            if i == j {
                continue;
            }
            let new_num = &rest_nums[0..j];
            let nm = (new_num.iter().join("") + &*cur_num.to_string() + &rest_nums[j..].iter().join("")).parse::<i64>().unwrap();
            rez.push((nm, i, j));
        }
    }
    let sorted_vec = rez.iter().sorted_by(|a, b| {
        return (a.0).cmp(&b.0);
    }).collect_vec();
    return **sorted_vec.get(0).unwrap();
}


#[cfg(test)]
mod tests {
    use super::*;

    fn testing(n: i64, exp: (i64, usize, usize)) -> () {
        let ans = smallest(n);
        assert_eq!(ans, exp, "Testing: {}", n);
    }

    #[test]
    fn basic_tests() {
        testing(261235, (126235, 2, 0));
        testing(209917, (29917, 0, 1));
        testing(285365, (238565, 3, 1));
    }
}

