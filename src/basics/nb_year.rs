fn nb_year(p0: i32, percent: f64, aug: i32, p: i32)-> i32 {

        let mut res = 0;
        let mut population = p0;
        let percent = (100.0 + percent) / 100.0;
        while population < p {
            population = (population as f64 * percent) as i32 + aug;
            res += 1;
        }
        res
}

#[cfg(test)]
mod tests {
    use super::*;

    fn dotest(p0: i32, percent: f64, aug: i32, p: i32, exp: i32) -> () {
        println!("p0: {:?};", p0);
        println!("percent: {:?};", percent);
        println!("aug: {:?};", aug);
        println!("p: {:?};", p);
        let ans =nb_year(p0, percent, aug, p);
        println!("actual:\n{:?};", ans);
        println!("expect:\n{:?};", exp);
        println!("{};", ans == exp);
        assert_eq!(ans, exp);
        println!("{};", "-");
    }

    #[test]
    fn basic_tests() {
        dotest(1500, 5.0, 100, 5000, 15);
      dotest(1500000, 2.5, 10000, 2000000, 10);
        dotest(18059, 0.05, 90, 1019139, 3611);

        /*
        p0: 18059;
percent: 0.05;
aug: 90;
p: 1019139;
actual:
3603;
expect:
3611;
false;
        */

    }
}