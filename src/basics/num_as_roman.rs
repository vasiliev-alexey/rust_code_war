/// Converts a number to a string representating roman numeral.
fn num_as_roman(num: i32) -> String {
    let mut num_to_calc = num;
    const ROMANS: [&str; 13] = ["M", "CM", "D", "CD", "C", "XC", "L", "XL", "X", "IX", "V", "IV", "I"];
    const NUMBERS: [i32; 13] = [1000, 900, 500, 400, 100, 90, 50, 40, 10, 9, 5, 4, 1];
    let mut res = "".to_string();

    while num_to_calc > 0 {
        for i in 0..ROMANS.len() {
            if num_to_calc >= NUMBERS[i] {
                num_to_calc -= NUMBERS[i];
                res = res  +  ROMANS[i];
                break;
            }
        }
    }

      res.to_string()
}

#[test]
fn returns_expected() {
    assert_eq!(num_as_roman(182), "CLXXXII");
    assert_eq!(num_as_roman(1990), "MCMXC");
    assert_eq!(num_as_roman(1875), "MDCCCLXXV");
}