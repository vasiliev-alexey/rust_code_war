use std::collections::HashMap;

fn cannons_ready(gunners: HashMap<&str, &str>) -> String {
    for x in gunners.iter() {
        if *x.1 == "nay" {
            return    "Shiver me timbers!".to_string();
        }
    }

    "Fire!".to_string()

}

#[cfg(test)]
mod tests {
    use std::collections::HashMap;
    use super::{cannons_ready};

    #[test]
    fn test_should_fire_the_cannons_when_ready() {
        assert_eq!(cannons_ready(HashMap::from([
            ("Mike", "aye"),
            ("Joe", "aye"),
            ("Johnson", "aye"),
            ("Peter", "aye"),
        ])), "Fire!");
    }

    #[test]
    fn test_should_shiver_your_timbers_instead_if_not_ready() {
        assert_eq!(cannons_ready(HashMap::from([
            ("Mike", "aye"),
            ("Joe", "nay"),
            ("Johnson", "aye"),
            ("Peter", "aye"),
        ])), "Shiver me timbers!");
    }
}