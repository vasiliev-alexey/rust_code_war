fn array_leaders(arr: &[i32]) -> Vec<i32> {
     let arr = arr.to_vec();
    let mut res =  vec![];
    let mut sum = 0;
    for x in arr.iter().rev() {

        if sum < *x {
            res.push(*x);
        }
        sum += x;

    }
    res.reverse();
    res
}
// https://www.codewars.com/kata/5a651865fd56cb55760000e0/train/rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn basic() {
        // Positive values
        assert_eq!(array_leaders(&[1,2,3,4,0]), [4]);
        assert_eq!(array_leaders(&[16,17,4,3,5,2]), [17,5,2]);

        // Negative values
        assert_eq!(array_leaders(&[-1,-29,-26,-2]), [-1]);
        assert_eq!(array_leaders(&[-36,-12,-27]),  [-36,-12]);

        // Mixed values
        assert_eq!(array_leaders(&[5,-2,2]), [5,2]);
        assert_eq!(array_leaders(&[0,-1,-29,3,2]),  [0,-1,3,2]);
    }
}
