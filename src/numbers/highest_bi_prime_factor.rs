use num::bigint::BigUint;
use num::ToPrimitive;


fn highest_bi_prime_factor(pp1: u32, pp2: u32, n: BigUint) -> (BigUint, u32, u32) {

    let min =  pp1.min(pp2);
 //   let res = [];
    let  mut tmp_max =BigUint::from(0u32);
    let  mut ret_i  = 0u32;
    let  mut ret_j  = 0u32;


  // let iter =  Math.ceil(Math.pow(n , 1/min));
    let iter = (n.nth_root( min  )).to_u32().unwrap().min(32);

  println!("{iter} , {}" , (1.0/min as f32));

    for i in 1.. iter {
        for j in 1.. iter{

            let  t = BigUint::from( pp1.saturating_pow( i) ) * BigUint::from(pp2.saturating_pow( j));
            if   t <= n &&   t >= tmp_max {
            //    println!("{i} , {j}");
                tmp_max = t;
                ret_i  = i;
                ret_j = j;

            }

        }
    }
    (tmp_max, ret_i, ret_j)
}


// Add your tests here.
// See https://doc.rust-lang.org/stable/rust-by-example/testing/unit_testing.html

#[cfg(test)]
mod tests {
    use num::bigint::{BigUint, ToBigUint};
    use super::highest_bi_prime_factor;

    fn dotest(p1: u32, p2: u32, n: BigUint, expected: (BigUint, u32, u32)) {
        let actual = highest_bi_prime_factor(p1, p2, n.clone());
        assert!(actual == expected,
                "With p1 = {p1}, p2 = {p2}, n = {n}\nExpected {expected:?} but got {actual:?}")
    }

    #[test]
    fn sample_tests() {
        // expect((highestBiPrimeFac(2, 31, 50000))).toEqual( [31744, 10, 1]);
      //  dotest(2, 31, 50000.to_biguint().unwrap(), (31744.to_biguint().unwrap(), 4, 1));
        dotest(2, 3, 50.to_biguint().unwrap(), (48.to_biguint().unwrap(), 4, 1));
        dotest(5, 11, 1000.to_biguint().unwrap(), (605.to_biguint().unwrap(), 1, 2));
        dotest(3, 13, 5000.to_biguint().unwrap(), (4563.to_biguint().unwrap(), 3, 2));
        dotest(5, 7, 5000.to_biguint().unwrap(), (4375.to_biguint().unwrap(), 4, 1));
    }
}
