fn camel_case(str: &str) -> String {
   let t = str.split_whitespace().map(| w|   {
        format!(
            "{}{}",
            &w[..1].to_uppercase(),
            &w[1..].to_lowercase()
        )
    }).collect::<Vec<String>>().join("");
    return t;
}

// Rust tests
#[test]
fn sample_test() {
    assert_eq!(camel_case("test case"), "TestCase");
    assert_eq!(camel_case("camel case method"), "CamelCaseMethod");
    assert_eq!(camel_case("say hello "), "SayHello");
    assert_eq!(camel_case(" camel case word"), "CamelCaseWord");
    assert_eq!(camel_case(""), "");
}

/*CamelCase Method
Write simple .camelCase method (camel_case function in PHP, CamelCase in C# or camelCase in Java) for strings. All words must have their first letter capitalized without spaces.

For instance:

camel_case("hello case"); // => "HelloCase"
camel_case("camel case word"); // => "CamelCaseWord"
Don't forget to rate this kata! Thanks :)

*/