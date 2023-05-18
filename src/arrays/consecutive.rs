use itertools::Itertools;

fn consecutive(xs: &[i16]) -> i16 {

    if xs.len() < 2 {
        return 0i16;
    }

   let v =  xs.iter().minmax();

    println!( "{:?}", v);

     return  v.into_option().unwrap().1 - v.into_option().unwrap().0 - xs.len() as i16 +1i16 as i16;


}
// https://www.codewars.com/kata/559cc2d2b802a5c94700000c/train/rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn basic() {
        assert_eq!(consecutive(&[4,8,6]), 2);
        assert_eq!(consecutive(&[1,2,3,4]), 0);
        assert_eq!(consecutive(&[]), 0);
        assert_eq!(consecutive(&[1]), 0);
    }
}
