fn to_camel_case(text: &str) -> String {

    // if text == "" {
    //    return  text.to_string();
    // }

  text.split(['_', '-'].as_ref()).collect::<Vec<_>>().iter().enumerate().map(| (i  , w)|   {
        if i !=0 {
               format!(
            "{}{}",
            &w[..1].to_uppercase(),
            &w[1..].to_lowercase()
        )
        } else {
            w.to_string()
        }

    }).collect::<Vec<String>>().join("")

  //  "".to_string()

}



// Add your tests here.
// See https://doc.rust-lang.org/stable/rust-by-example/testing/unit_testing.html

#[cfg(test)]
mod tests {
    use super::to_camel_case;

    const ERR_MSG: &str = "\nYour result (left) did not match the expected output (right)";

    fn dotest(s: &str, expected: &str) {
        assert_eq!(to_camel_case(s), expected, "{ERR_MSG} with text = \"{s}\"")
    }

    #[test]
    fn fixed_tests() {
       dotest("","");
        dotest("the_stealth_warrior", "theStealthWarrior");
         dotest("The-Stealth-Warrior", "TheStealthWarrior");
        dotest("A-B-C", "ABC");
    }
}

/*
https://www.codewars.com/kata/517abf86da9663f1d2000003/train/rust
*/

