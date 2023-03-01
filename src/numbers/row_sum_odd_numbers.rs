fn row_sum_odd_numbers(n: i64) -> i64 {
    let c = n - 1;
    let b = ((1 + 1 + 2 * c) / 2) * c + 1;
    let mut rez = b;
    for index in 1..c + 1 {
        rez = rez + b + 2 * index;
    }
    rez
}

/*

  const c = n - 1;
  let b = ((1 + 1 + 2 * c) / 2) * c + 1;
  let rez = b;
  for (let index = 1; index <= c; index++) {
    rez = rez + b + 2 * index;
  }
  return rez;*/

#[test]
fn returns_expected() {
    assert_eq!(row_sum_odd_numbers(1), 1);
    assert_eq!(row_sum_odd_numbers(42), 74088);
}

/*
Given the triangle of consecutive odd numbers:

             1
          3     5
       7     9    11
    13    15    17    19
21    23    25    27    29

Calculate the sum of the numbers in the nth row of this triangle (starting at index 1) e.g.: (Input --> Output)

1 -->  1
2 --> 3 + 5 = 8

*/