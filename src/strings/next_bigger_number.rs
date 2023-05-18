use itertools::Itertools;

fn next_bigger_number(n: i64) -> i64 {
    let digits = n.to_string().chars().map(|x| x.to_digit(10).unwrap()).collect::<Vec<_>>();
    let mut sorted = digits.clone();
    sorted.sort_by(|a, b| b.cmp(a));

    //
    if digits.eq(&sorted) {
        return -1;
    }
    for i in (1..digits.len()).rev() {
        if digits[i] > digits[i - 1] {
            let num_ord = digits[i - 1];
            let digs = digits[i..].to_vec();

            let mut filtered_rest = digs.iter().filter(|e| e > &&num_ord).collect::<Vec<_>>();

            filtered_rest.sort();
            let next_max = filtered_rest.get(0).unwrap().clone();
            let index = digs.iter().position(|&r| r == *next_max).unwrap();

            let mut rest_digits = digs.clone();
            rest_digits[index] = num_ord;
            rest_digits.sort();

            let mut head = digits[0..i - 1].iter().join("");
            head.push_str(&next_max.to_string());
            head.push_str(&rest_digits.iter().join(""));

            return head.parse::<i64>().unwrap();
        }
    }


    0
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn basic() {
        assert_eq!(-1, next_bigger_number(9));
        assert_eq!(-1, next_bigger_number(111));
        assert_eq!(-1, next_bigger_number(531));
        assert_eq!(21, next_bigger_number(12));
        assert_eq!(531, next_bigger_number(513));
        assert_eq!(2071, next_bigger_number(2017));
        assert_eq!(441, next_bigger_number(414));
        assert_eq!(414, next_bigger_number(144));
        assert_eq!(1234567908, next_bigger_number(1234567890));
        //        // assert.strictEqual(nextBigger(1234567890), 1234567908);
    }
}

/* Next bigger number with the same digits

Create a function that takes a positive integer and returns the next bigger number that can be formed by rearranging its digits. For example:

12 ==> 21
513 ==> 531
2017 ==> 2071
nextBigger(num: 12)   // returns 21
nextBigger(num: 513)  // returns 531
nextBigger(num: 2017) // returns 2071
If the digits can't be rearranged to form a bigger number, return -1 (or nil in Swift):

9 ==> -1
111 ==> -1
531 ==> -1
nextBigger(num: 9)   // returns nil
nextBigger(num: 111) // returns nil
nextBigger(num: 531) // returns nil

*/