use itertools::Itertools;

fn xo(string: &'static str) -> bool {

   let xo =  string.to_lowercase().chars().filter( |x| x== &'x' || x== &'o').collect_vec();

    let mut x_cnt = 0;
    let mut o_cnt = 0;

    for x in xo {

      match  x {
          'o' => o_cnt+=1,
          'x' =>x_cnt+=1,
          _ => unreachable!()

      }

    }
    if o_cnt==x_cnt {
        return  true;
    }
    false
}

#[test]
fn returns_expected() {
    assert_eq!(xo("xo"), true);
    assert_eq!(xo("Xo"), true);
    assert_eq!(xo("xxOo"), true);
    assert_eq!(xo("xxxm"), false);
    assert_eq!(xo("Oo"), false);
    assert_eq!(xo("ooom"), false);
}