fn paperwork(n: i16, m: i16) -> u32 {
    if n  <0 || m < 0 {
        return  0;
    }
    (n * m) as u32
}

// Add your tests here.
// See https://doc.rust-lang.org/stable/rust-by-example/testing/unit_testing.html
// https://www.codewars.com/kata/55f9b48403f6b87a7c0000bd/train/rust
#[cfg(test)]
mod tests {
    use super::paperwork;

    fn dotest(n: i16, m: i16, expected: u32) {
        let actual = paperwork(n, m);
        assert!(actual == expected,
                "With n = {n}, m = {m}\nExpected {expected} but got {actual}")
    }

    #[test]
    fn test_paperwork() {
        dotest(5,5, 25);
        dotest(5,-5, 0);
        dotest(-5,-5, 0);
        dotest(-5,5, 0);
        dotest(5,0, 0);
    }

}



fn maps(values: &Vec<i32>) -> Vec<i32> {
    values.iter().map( |x| x * 2).collect()
}

macro_rules! compare {
  ( $got : expr, $expect : expr ) => {
    if $got != $expect { panic!("Got: {:?}\nExpect: {:?}\n", $got, $expect); }
  };
}

#[test]
fn sample_tests() {
    compare!(maps(&vec![1, 2, 3, 4]),                        vec![2, 4, 6, 8]);
    compare!(maps(&vec![12, 5, 9, 7]),                       vec![24, 10, 18, 14]);
    compare!(maps(&vec![19037, 2793, 345, 544, 43]),         vec![38074, 5586, 690, 1088, 86]);
    compare!(maps(&vec![-7, -43, -98, -45, -32, -1123, -1]), vec![-14, -86, -196, -90, -64, -2246, -2]);
    compare!(maps(&vec![7, -43, -98, 45, 32, 0, -1, 3]),     vec![14, -86, -196, 90, 64, 0, -2, 6]);
    compare!(maps(&vec![]),                                  vec![] as Vec<i32>);
}
