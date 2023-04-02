fn hello(name: &str) -> String {
   let mut name  = name.to_lowercase();
    if  name == "" {
        name = "World".to_string();
    }
    let  name = name.chars().next().unwrap().to_uppercase().to_string() + &name[1..];
    format!("Hello, {}!", name)

}

// Add your tests here.
// See https://doc.rust-lang.org/stable/rust-by-example/testing/unit_testing.html

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_basic() {
        assert_eq!(hello("johN"), "Hello, John!");
        assert_eq!(hello("alice"), "Hello, Alice!");
        assert_eq!(hello(""), "Hello, World!");
    }
}
