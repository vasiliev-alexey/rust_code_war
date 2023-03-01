fn nb_dig(n: i32, d: i32) -> i32 {
    let ch = d.to_string();
    let rez = (0..=n).map(|v| (v * v).to_string()).fold(0, |acc, v| acc + v.matches(&ch).count() as i32)
        ;
    rez
}

#[cfg(test)]
mod tests {
    use super::*;

    fn dotest(n: i32, d: i32, exp: i32) -> () {
        println!("n: {:?}", n);
        println!("d: {:?}", d);
        let ans = nb_dig(n, d);
        println!("actual:\n{:?}", ans);
        println!("expect:\n{:?}", exp);
        println!("{}", ans == exp);
        assert_eq!(ans, exp);
        println!("{}", "-");
    }

    #[test]
    fn basic_tests() {
        //   dotest(25, 1, 213);
        dotest(550, 5, 213);
        // dotest(5750, 0, 4700);
    }
}

//https://www.codewars.com/kata/566fc12495810954b1000030/train/rust
/*
Take an integer n (n >= 0) and a digit d (0 <= d <= 9) as an integer.
Square all numbers k (0 <= k <= n) between 0 and n.
Count the numbers of digits d used in the writing of all the k**2.

Implement the function taking n and d as parameters and returning this count.
Examples:
n = 10, d = 1
the k*k are 0, 1, 4, 9, 16, 25, 36, 49, 64, 81, 100
We are using the digit 1 in: 1, 16, 81, 100. The total count is then 4.

The function, when given n = 25 and d = 1 as argument, should return 11 since
the k*k that contain the digit 1 are:
1, 16, 81, 100, 121, 144, 169, 196, 361, 441.
So there are 11 digits 1 for the squares of numbers between 0 and 25.
Note that 121 has twice the digit 1.

*/