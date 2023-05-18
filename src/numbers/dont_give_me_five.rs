fn dont_give_me_five(start: isize, end: isize) -> isize {
let mut  rez = 0;
    for i in start..=end {
        if !i.to_string().contains("5") {
            rez = rez +1;
        }
    }

    rez
}

// Rust test example:
// TODO: replace with your own tests (TDD), these are just how-to examples.
// See: https://doc.rust-lang.org/book/testing.html

#[test]
fn returns_expected() {
    assert_eq!(dont_give_me_five(1, 9), 8);
    assert_eq!(dont_give_me_five(4, 17), 12);
}