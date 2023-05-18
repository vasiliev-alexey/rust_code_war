use itertools::Itertools;

fn bubblesort_once(lst: &[u32]) -> Vec<u32> {

    let mut  rez =  lst.iter().map(|v| *v).collect_vec();
    for i in 0..rez.len()-1 {
        if rez[i] <= rez[i + 1] {
            continue
        }
        let a = rez[i];
        rez[i] = rez[i+1];
        rez[i+1] = a;

    }
    rez

}


// Add your tests here.
// See https://doc.rust-lang.org/stable/rust-by-example/testing/unit_testing.html

#[cfg(test)]
mod tests {
    use super::bubblesort_once;

    fn dotest(a: &[u32], expected: &[u32]) {
        let actual = bubblesort_once(a);
        assert!(actual == expected,
                "With a = {a:?}\nExpected {expected:?} but got {actual:?}")
    }

    #[test]
    fn example_test() {
        dotest(&[9, 7, 5, 3, 1, 2, 4, 6, 8], &[7, 5, 3, 1, 2, 4, 6, 8, 9]);
    }
}


