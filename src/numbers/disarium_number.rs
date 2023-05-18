fn disarium_number(n: u32) -> String {
    let  mut rez = 0u32;
    for (i , x) in n.to_string().chars().map(|x| x.to_digit(10).unwrap() as u32).enumerate() {
        rez = rez + x.pow(i as u32+1);
    }

    return if rez == n { "Disarium !!".to_string() } else { "Not !!".to_string() }
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn basic() {
        assert_eq!(disarium_number(89),"Disarium !!");
        // assert_eq!(disarium_number(564),"Not !!");
        // assert_eq!(disarium_number(1024),"Not !!");
        // assert_eq!(disarium_number(135),"Disarium !!");
        // assert_eq!(disarium_number(136586),"Not !!");
    }
}
