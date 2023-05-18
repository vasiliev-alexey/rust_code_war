use itertools::Itertools;

fn barista(coffees: Vec<u8>) -> u16 {

    let mut rez = vec![0];
    let coffees = coffees.iter().sorted_by(|x, y| x.cmp(y)).map(|e | *e as u16).collect::<Vec<u16>>();

    for (i , x) in coffees.iter().enumerate() {
        println!("x {:?}" ,   x);
          rez.push(rez[rez.len() - 1] + if i == 0 { 0 } else { 2 } + x) ;
    }
   return   rez.iter().sum::<u16>();

}

#[cfg(test)]
mod tests {
    use super::barista;

    #[test]
    fn sample_tests() {
        assert_eq!(barista(vec![]), 0);
        assert_eq!(barista(vec![2, 10, 5, 3, 9]), 85);
        assert_eq!(barista(vec![4, 3, 2]), 22);
        assert_eq!(barista(vec![20, 5]), 32);
        assert_eq!(barista(vec![20, 5, 4, 3, 1, 5, 7, 8]), 211);
        assert_eq!(barista(vec![5, 4, 3, 2, 1]), 55);
    }
}