use itertools::Itertools;
use num::bigint::BigUint;


fn sum_of_digits ( n : &BigUint)  -> u32 {
    n.to_string().chars().map(  |c| c.to_digit(10).unwrap()).fold(0, | x , y| x+y) //.fold( 0 | (x, y)| x+y)
}

fn power_sum_dig_term(n: u32) -> BigUint {
    let mut res: Vec<BigUint> = vec![];
     for i in 2..500 as u32 {
        for j in 2.. 50 as u32 {
             let  product = BigUint::from(i).pow(j);

            if sum_of_digits(&product) == i {
                res.push(product.clone());

            }
        }
        

    }

    res =  res.iter().sorted_by( |x , y| x.cmp(y)).map( |v| v.clone()).collect_vec();
    println!("{res:?}");
   return  res.get(n as usize -1).unwrap().clone();

}

//   Condition: https://www.codewars.com/kata/55f4e56315a375c1ed000159


// Add your tests here.
// See https://doc.rust-lang.org/stable/rust-by-example/testing/unit_testing.html

#[cfg(test)]
mod tests {

    use num::bigint::{BigUint, ToBigUint};
    use crate::numbers::power_sum_dig_term::power_sum_dig_term;

    fn dotest(n: u32, expected: BigUint) {
        let actual = power_sum_dig_term(n);
        assert!(actual == expected,
                "With n = {n}\nExpected {expected} but got {actual}")
    }

    #[test]
    fn fixed_tests() {
        dotest(1, 81.to_biguint().unwrap());
        dotest(2, 512.to_biguint().unwrap());
        dotest(3, 2401.to_biguint().unwrap());
        dotest(4, 4913.to_biguint().unwrap());
        dotest(5, 5832.to_biguint().unwrap());
    }
}
