use itertools::Itertools;

pub fn range_extraction(a: &[i32]) -> String {

    let mut rez: Vec<Vec<i32>> = vec![];
    let mut tmp: Vec<i32> = vec![];
    tmp.push(a[0]);

    for i in 1..a.len() {
        if a[i] - a[i - 1] > 1 {
            rez.push(tmp);
            tmp = vec![];
            tmp.push(a[i]);
        } else {
            tmp.push(a[i])
        }
    }
    rez.push(tmp);




    let mut rez2: Vec<Vec<i32>> = vec![];
    for t in rez {

        let min_bound = *t.iter().min().unwrap();
        let max_bound = *t.iter().max().unwrap();

        if t.len() == 2 {
            rez2.push(vec![min_bound]);
            rez2.push(vec![max_bound]);
        } else if t.len() > 2 {
            rez2.push(vec![min_bound, max_bound])
        } else {
            rez2.push(vec![min_bound]);
        }
    }
    rez2.iter().map(|e| e.iter().join("-").to_string()).join(",")
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example() {
        assert_eq!(range_extraction(&[-6, -3, -2, -1, 0, 1, 3, 4, 5, 7, 8, 9, 10, 11, 14, 15, 17, 18, 19, 20]), "-6,-3-1,3-5,7-11,14,15,17-20");
        assert_eq!(range_extraction(&[-3, -2, -1, 2, 10, 15, 16, 18, 19, 20]), "-3--1,2,10,15,16,18-20");
    }
}


/* Range Extraction
https://www.codewars.com/kata/51ba717bb08c1cd60f00002f/train/rust
A format for expressing an ordered list of integers is to use a comma separated list of either

    individual integers
    or a range of integers denoted by the starting integer separated from the end integer in the range by a dash, '-'. The range includes all integers in the interval including both endpoints. It is not considered a range unless it spans at least 3 numbers. For example "12,13,15-17"

Complete the solution so that it takes a list of integers in increasing order and returns a correctly formatted string in the range format.

Example:

solution([-10, -9, -8, -6, -3, -2, -1, 0, 1, 3, 4, 5, 7, 8, 9, 10, 11, 14, 15, 17, 18, 19, 20]);
// returns "-10--8,-6,-3-1,3-5,7-11,14,15,17-20"

Courtesy of rosettacode.org

*/