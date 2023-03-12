fn snail(matrix: &[Vec<i32>]) -> Vec<i32> {
    let size = matrix.len();
    if size == 0 {
        return Vec::new();
    }
    if size == 1 {
        return Vec::from([1]);
    }
    let top = &matrix[0][0..matrix[0].len() - 1];
    let right = matrix.slice(0, -1).map((a) => {
        return a[a.length - 1];
    });
    println!("{:?}", top);
    // enjoy
    vec![]
}

/*
    const size = array.length;
    if (size == 0) return [];
    if (size == 1) return array[0];
    const top = array[0].slice(0, -1);

    const right = array.slice(0, -1).map((a) => {
        return a[a.length - 1];
    });
    const bottom = array[size - 1].slice(1).reverse();
    const left = array
        .slice(1)
        .map((a) => a[0])
        .reverse();
    const inner = array.slice(1, -1).map((a) => a.slice(1, -1));
    const rez  = [];
    return rez.concat(top, right, bottom, left, snail(inner));
*/


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sample_test1() {
        let square = &[
            vec![1, 2, 3],
            vec![4, 5, 6],
            vec![7, 8, 9],
        ];
        let expected = vec![1, 2, 3, 6, 9, 8, 7, 4, 5];
        assert_eq!(snail(square), expected);
    }

    #[test]
    fn sample_test2() {
        let square = &[
            vec![1, 2, 3],
            vec![8, 9, 4],
            vec![7, 6, 5],
        ];
        let expected = vec![1, 2, 3, 4, 5, 6, 7, 8, 9];
        assert_eq!(snail(square), expected);
    }

    #[test]
    fn sample_test3() {
        let square: &[Vec<i32>; 1] = &[Vec::new()];
        let expected = Vec::new();
        assert_eq!(snail(square), expected, "Failed with empty input");
    }

    #[test]
    fn sample_test4() {
        let square = &[vec![1]];
        let expected = vec![1];
        assert_eq!(snail(square), expected);
    }
}


/*
Snail

https://www.codewars.com/kata/521c2db8ddc89b9b7a0000c1/train/rust
Snail Sort
Given an n x n array, return the array elements arranged from outermost elements to the middle element, traveling clockwise.

array = [[1,2,3],
         [4,5,6],
         [7,8,9]]
snail(array) #=> [1,2,3,6,9,8,7,4,5]
For better understanding, please follow the numbers of the next array consecutively:

array = [[1,2,3],
         [8,9,4],
         [7,6,5]]
snail(array) #=> [1,2,3,4,5,6,7,8,9]
This image will illustrate things more clearly:


NOTE: The idea is not sort the elements from the lowest value to the highest; the idea is to traverse the 2-d array in a clockwise snailshell pattern.

NOTE 2: The 0x0 (empty matrix) is represented as en empty array inside an array [[]].


*/