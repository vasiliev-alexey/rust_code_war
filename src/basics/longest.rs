use itertools::Itertools;

fn longest(a1: &str, a2: &str) -> String {


    let mut res = vec![];

    for x in a1.chars() {
        res.push(x);
    }

    for x in a2.chars() {
        res.push(x);
    }

    res.iter().sorted().unique().join("")


    //     println!("--{res:?}");
    // "".to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    fn testing(s1: &str, s2: &str, exp: &str) -> () {
        println!("s1:{:?} s2:{:?}", s1, s2);
        println!("{:?} {:?}", longest(s1, s2), exp);
        println!("{}", longest(s1, s2) == exp);
        assert_eq!(&longest(s1, s2), exp)
    }

    #[test]
    fn basic_tests() {
        testing("aretheyhere", "yestheyarehere", "aehrsty");
        testing("loopingisfunbutdangerous", "lessdangerousthancoding", "abcdefghilnoprstu");
    }
}
