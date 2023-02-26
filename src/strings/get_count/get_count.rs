fn get_count(string: &str) -> usize {
    string.chars().filter( |x| ['a', 'e', 'i', 'o', 'u'].contains(&x)).count()
}


#[test]
fn my_tests() {
    assert_eq!(get_count("abracadabra"), 5);
}