use std::collections::HashMap;
use itertools::Itertools;

fn solve( vec: &[i32]) -> Vec<i32> {
    let mut rez = Vec::from(vec);
    let  mut map_fr =HashMap::new();

    for x in &rez {
       *map_fr.entry(x).or_insert(0) +=1;

    }

    rez = rez.iter().sorted_by( |x, y|   {

        if  (map_fr.get(x).unwrap() - map_fr.get(y).unwrap()) == 0 {
            return  x.cmp(y)
        } else {
            return  map_fr.get(y).unwrap().cmp( map_fr.get(x).unwrap())
        }

    }).map( |c| *c).collect_vec();
    rez

}

#[cfg(test)]
mod tests {
    use super::solve;

    #[test]
    fn basic_tests() {
        assert_eq!(solve(&vec![2,3,5,3,7,9,5,3,7]), vec![3,3,3,5,5,7,7,2,9]);
        assert_eq!(solve(&vec![1,2,3,0,5,0,1,6,8,8,6,9,1]), vec![1,1,1,0,0,6,6,8,8,2,3,5,9]);
        assert_eq!(solve(&vec![5,9,6,9,6,5,9,9,4,4]), vec![9,9,9,9,4,4,5,5,6,6]);
        assert_eq!(solve(&vec![4,4,2,5,1,1,3,3,2,8]), vec![1,1,2,2,3,3,4,4,5,8]);
        assert_eq!(solve(&vec![4,9,5,0,7,3,8,4,9,0]), vec![0,0,4,4,9,9,3,5,7,8]);
    }
}