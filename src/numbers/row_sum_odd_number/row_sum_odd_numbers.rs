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