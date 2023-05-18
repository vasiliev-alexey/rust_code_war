fn all_non_consecutive(arr: &[i32]) -> Vec<(usize, i32)> {
    /* return arr.reduce((acc, number, index) => {
        const nextIndex = index + 1
        const nextNumber = arr[nextIndex]

        if (typeof nextNumber === 'undefined' || number + 1 === nextNumber) return acc
        return [...acc, { i: nextIndex, n: nextNumber }]
    }, [])*/

  let  mut rez : Vec<(usize, i32)> = vec![];

    for i in 1.. arr.len() {
        if arr[i-1] +1 != arr[i] {
            rez.push((i, arr[i]));
        }
    }
    
    return rez;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_simple() {
        let expect: Vec<(usize, i32)> = vec![
            (4, 6), (7, 10),
        ];
        let result = all_non_consecutive(&[1, 2, 3, 4, 6, 7, 8, 10]);

        assert_eq!(expect, result);
    }
}

// https://www.codewars.com/kata/58f8b35fda19c0c79400020f/train/rust
/*
Find all non-consecutive numbers
Your task is to find all the elements of an array that are non consecutive.

A number is non consecutive if it is not exactly one larger than the previous element in the array. The first element gets a pass and is never considered non consecutive.

Create a function name all_non_consecutive

E.g., if we have an array [1,2,3,4,6,7,8,15,16] then 6 and 15 are non-consecutive.

You should return the results as an array of tuples with two values: the index of the non-consecutive number and the non-consecutive number.

E.g., for the above array the result should be:
*/