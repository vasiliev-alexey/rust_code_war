fn greet(name: &str) -> String {
    let mut c = name.chars();
   let name_cap  =  match    c.next() {
        None => String::new(),
        Some(f) => f.to_uppercase().collect::<String>() + c.as_str().to_lowercase().as_str(),
    };
    format!("Hello {}!", name_cap)

}
/*
fn greet(name: &str) -> String {
    format!(
        "Hello {}{}!",
        &name[..1].to_uppercase(),
        &name[1..].to_lowercase()
    )
}
*/

#[cfg(test)]
mod tests {
    use super::greet;

    const ERR: &str = "\nYour result (left) did not match the expected output (right).";

    #[test]
    fn returns_expected() {
        assert_eq!(greet("riley"), "Hello Riley!", "{ERR}");
        assert_eq!(greet("JACK"), "Hello Jack!", "{ERR}");
    }
}

/*
Greet Me
Write a method that takes one argument as name and then greets that name, capitalized and ends with an exclamation point.
Example:
"riley" --> "Hello Riley!"
"JACK"  --> "Hello Jack!"
*/