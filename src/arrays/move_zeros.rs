fn move_zeros(arr: &[u8]) -> Vec<u8> {
    let mut  rez = arr.iter().filter(|x| **x != ( 0 as u8)).map( |x| *x).collect::<Vec<u8>>() ;
    for _ in 0.. arr.len() - rez.len()  {
        rez.push(0);
    }
    rez
}
// Add your tests here.
// See https://doc.rust-lang.org/stable/rust-by-example/testing/unit_testing.html

/*
let mut ys = Vec::with_capacity(xs.len());
    ys.extend(xs.iter().filter(|&&x| x != 0));
    ys.resize(xs.len(), 0);
    ys
*/

#[cfg(test)]
mod tests {
    use super::move_zeros;

   // const ERR_MSG: &str = "\nYour result (left) did not match the expected output (right)";

    fn dotest(a: &[u8], expected: &[u8]) {
        let actual = move_zeros(a);
        assert!(actual == expected, "With arr = {a:?}\nExpected {expected:?} but got {actual:?}")
    }

    #[test]
    fn sample_tests() {
        dotest(&[1, 2, 0, 1, 0, 1, 0, 3, 0, 1], &[1, 2, 1, 1, 3, 1, 0, 0, 0, 0]);
        dotest(&[9, 0, 0, 9, 1, 2, 0, 1, 0, 1, 0, 3, 0, 1, 9, 0, 0, 0, 0, 9], &[9, 9, 1, 2, 1, 1, 3, 1, 9, 9, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0]);
        dotest(&[0, 0], &[0, 0]);
        dotest(&[0], &[0]);
        dotest(&[], &[]);
    }
}


/*
Moving Zeros To The End

Write an algorithm that takes an array and moves all of the zeros to the end, preserving the order of the other elements.

moveZeros([false,1,0,1,2,0,1,3,"a"]) // returns[false,1,1,2,1,3,"a",0,0]
https://www.codewars.com/kata/52597aa56021e91c93000cb0/train/rust
*/