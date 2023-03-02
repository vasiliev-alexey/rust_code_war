
fn multiplication_table(len: usize) -> Vec<Vec<usize>> {
     let mut rez : Vec<Vec<usize>> =vec![];
    for i in 1.. len+1{
        let mut  tmp = vec![];
        for j in 1.. len+1 {

            tmp.push(i*j);
        }
        rez.push(tmp);
    }
    rez
}


//    (1..=n).map(|i| (1..=n).map(|j| i*j).collect()).collect()
// Add your tests here.
// See https://doc.rust-lang.org/stable/rust-by-example/testing/unit_testing.html

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn basic() {
        assert_eq!(multiplication_table(3), [[1,2,3], [2,4,6], [3,6,9]]);
    }
}


/*Multiplication table
Your task, is to create N×N multiplication table, of size provided in parameter.

For example, when given size is 3:

1 2 3
2 4 6
3 6 9

For the given example, the return value should be:

[[1,2,3],[2,4,6],[3,6,9]]

*/