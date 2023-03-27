fn count_spec_mult(n: u8, max_val: u64) -> u64 {
    let prim  = vec! [2, 3, 5, 7, 11, 13, 17, 19, 23, 29, 31, 37];
    let mut  prod = 1;
    for i in 0..n  {
        prod =  prod *  prim.get(i as usize).unwrap();
    }
    println!("{}",  max_val / prod);
      //  const prod = prime.slice(0, n).reduce((a, b) => a * b, 1)
    //   return ~~(mxval / prod)
    max_val / prod
        //return ~~(mxval / prod)
}


// Add your tests here.
// See https://doc.rust-lang.org/stable/rust-by-example/testing/unit_testing.html

#[cfg(test)]
mod tests {
    use super::count_spec_mult;

    #[test]
    fn fixed_tests() {
        assert_eq!(count_spec_mult(3, 200), 6);
        assert_eq!(count_spec_mult(3, 1000), 33);
        assert_eq!(count_spec_mult(4, 500), 2);
        assert_eq!(count_spec_mult(4, 1000), 4);
    }
}


/* Special Multiples
Some numbers have the property to be divisible by 2 and 3. Other smaller subset of numbers have the property to be divisible by 2, 3 and 5. Another subset with less abundant numbers may be divisible by 2, 3, 5 and 7. These numbers have something in common: their prime factors are contiguous primes.

Implement a function that finds the amount of numbers that have the first N primes as factors below a given limit.

Let's see some cases:

count_specMult(3, 200)  =>  6

The first 3 primes are: 2, 3 and 5.

And the found numbers below 200 are: 30, 60, 90, 120, 150, 180.

Happy coding!!

*/