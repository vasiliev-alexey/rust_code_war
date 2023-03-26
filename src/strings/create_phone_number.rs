use itertools::Itertools;

fn create_phone_number(numbers: &[u8]) -> String {
    let f = [numbers[0], numbers[1], numbers[2]];
    let s = [numbers[3], numbers[4], numbers[5]];
    let t = [numbers[6], numbers[7], numbers[8], numbers[9]];
    format!("({}) {}-{}",
            f.iter().map(|x| x.to_string()).join(""),
            s.iter().map(|x| x.to_string()).join(""),
            t.iter().map(|x| x.to_string()).join(""))
}


#[test]
fn returns_expected() {
    assert_eq!(create_phone_number(&[1, 2, 3, 4, 5, 6, 7, 8, 9, 0]), "(123) 456-7890");
    assert_eq!(create_phone_number(&[1, 1, 1, 1, 1, 1, 1, 1, 1, 1]), "(111) 111-1111");
    assert_eq!(create_phone_number(&[1, 2, 3, 4, 5, 6, 7, 8, 9, 9]), "(123) 456-7899");
}

