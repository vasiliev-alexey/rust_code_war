fn alex_mistakes(number_of_katas: u32, time_limit: u32) -> u32 {
    let mut limit_time =  time_limit;
    const MINUTES_PER_KATA: u32 = 6;
    let mut pushup_time = 5;
    let mut pushup_count = 0;

    while number_of_katas * MINUTES_PER_KATA + pushup_time <= limit_time {
        pushup_count+=1;
        limit_time -= pushup_time;
        pushup_time *= 2;
    }

    return pushup_count;
}


// Add your tests here.
// See https://doc.rust-lang.org/stable/rust-by-example/testing/unit_testing.html

#[cfg(test)]
mod tests {
    use super::alex_mistakes;

    fn dotest(k: u32, t:u32, expected: u32) {
        let actual = alex_mistakes(k, t);
        assert!(actual == expected, "With number_of_katas = {k}, time_limit = {t}\nExpected {expected} but got {actual}")
    }

    #[test]
    fn fixed_tests() {
        dotest(10, 120, 3);
        dotest(11, 120, 3);
        dotest(3, 45, 2);
        dotest(8, 120, 3);
        dotest(6, 60, 2);
        dotest(9, 180, 4);
        dotest(20, 120, 0);
        dotest(20, 125, 1);
        dotest(20, 130, 1);
        dotest(20, 135, 2);
    }
}
