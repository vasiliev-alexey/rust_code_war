use itertools::Itertools;

fn sort_array(arr: &[i32]) -> Vec<i32> {

    //  let oddArray = array.filter((e, i) => e % 2 != 0).sort((a, b) => b - a);
    //  return array.map((e, i) => e % 2 != 0 ? oddArray.pop() : e);

    let mut odd_vec = arr.iter().filter(|e| *e % 2 != 0).sorted_by(|a, b| Ord::cmp(b, a)).map(|e| *e).collect::<Vec<i32>>();
    arr.iter().map(|e| if  e % 2 != 0 { odd_vec.pop().unwrap() } else { *e }).collect()

}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn basic() {
        assert_eq!(sort_array(&[5, 3, 2, 8, 1, 4]), [1, 3, 2, 8, 5, 4]);
        assert_eq!(sort_array(&[5, 3, 1, 8, 0]), [1, 3, 5, 8, 0]);
        assert_eq!(sort_array(&[]), []);
    }
}

/* Sort the odd
Task
You will be given an array of numbers. You have to sort the odd numbers in ascending order while leaving the even numbers at their original positions.

Examples
[7, 1]  =>  [1, 7]
[5, 8, 6, 3, 4]  =>  [3, 8, 6, 5, 4]
[9, 8, 7, 6, 5, 4, 3, 2, 1, 0]  =>  [1, 8, 3, 6, 5, 4, 7, 2, 9, 0]

*/