use itertools::Itertools;

fn clean_string(s: &str) -> String {
    // your code here

     let  mut chars =  vec![];

    for x in s.chars() {
        if x != '#' {
            chars.push(x)
        } else {
            chars.pop();
        }
    }
    println!("{chars:?}");

    chars.iter().join("").to_string()
}

//https://www.codewars.com/kata/5727bb0fe81185ae62000ae3/train/rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sample_tests() {
        assert_eq!(clean_string(""), "");
        assert_eq!(clean_string("abc####d##c#"), "");
        assert_eq!(clean_string("abc#d##c"), "ac");

        assert_eq!(clean_string("#######"), "");

    }
}
