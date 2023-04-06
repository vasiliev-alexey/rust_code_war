use itertools::Itertools;

fn print(n: i32) -> Option<String> {

    if n <= 0 || n % 2 == 0 {
        return None;
    }

    if n == 1 { return Some("*\n".to_string()); }

    let x = (n + 1) / 2;
    let mut arr = vec![];

    for i in 1..x {
        let space = " ".repeat((x - i) as usize);
        let diamand = "*".repeat((2 * i - 1) as usize );
        arr.push( space + &*diamand + "\n");
    }
    let mut top = arr.iter().join("");
    let  bottom = arr.iter().rev().join("");
    return Some(top + &*"*".repeat(n as usize) + "\n" + &bottom);
}

#[test]
fn basic_test() {

    assert_eq!(print(-3),None);
    assert_eq!(print(2),None);
    assert_eq!(print(0),None);
     assert_eq!(print(3), Some(" *\n***\n *\n".to_string()) );
    assert_eq!(print(5), Some("  *\n ***\n*****\n ***\n  *\n".to_string()));
    assert_eq!(print(1), Some("*\n".to_string()));
}