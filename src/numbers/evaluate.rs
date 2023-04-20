use itertools::Itertools;

fn evaluate(equation: String) -> Option<i64> {

    fn oper   (a : i64 ,b : i64) -> i64 {      (a + b) + (a - b) + (a * b) + (a / b)}


    let rr =  equation.split("@").collect_vec();
    println!("{rr:?}");

    let  ar = equation.split("@").map(|x| x.trim().parse::<i64>().unwrap()).collect_vec();
    println!("{ar:?}");
    let mut st = *ar.get(0).unwrap();

    for i in 1.. ar.len() {
        if ar[i] == 0 {
            return  None;
        }
        st = oper(st , ar[i]);
    }
    println!("{st:?}");
    Some(st)
}

// Add your tests here.
// See https://doc.rust-lang.org/stable/rust-by-example/testing/unit_testing.html

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        [
            ("1 @ 1", Some(4)),
            ("3 @ 2", Some(13)),
            ("6 @ 9", Some(66)),
            ("4 @ -4", Some(-9)),
            ("1 @ 1 @ -4", Some(-9)),
            ("2 @ 2 @ 2", Some(40)),
            ("0 @ 1 @ 2", Some(0)),
            ("5 @ 0", None)
        ].into_iter()
            .for_each(|(eq, res)| {
                assert_eq!(evaluate(eq.to_string()), res);
            });
    }
}
