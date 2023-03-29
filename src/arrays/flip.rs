use itertools::Itertools;

fn flip(dir: char, cubes: &[u32]) -> Vec<u32> {
      cubes.iter().sorted_by( |a , b|  {
        if dir == 'R' {
             a.cmp( b)
        } else  {
            b.cmp( a)
        }
    }).map(|x| *x).collect::<Vec<u32>>()
}


// https://www.codewars.com/kata/5f70c883e10f9e0001c89673/train/rust
#[cfg(test)]
mod tests {
    use super::flip;

    #[test]
    fn sample_tests() {
        assert_eq!(flip('R', &vec![3, 2, 1, 2]), vec![1, 2, 2, 3]);
        assert_eq!(flip('L', &vec![1, 4, 5, 3, 5]), vec![5, 5, 4, 3, 1]);
    }
}
