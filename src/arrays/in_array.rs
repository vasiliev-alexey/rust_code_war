use std::collections::HashSet;
use itertools::Itertools;

fn in_array(arr_a: &[&str], arr_b: &[&str]) -> Vec<String> {
  // return array1.filter(substr => array2.some(word => word.includes(substr))).sort()

    let  mut  rez : HashSet<String> = HashSet::new();

    for x in arr_a {

        if arr_b.iter().filter( |w| w.contains(x)).collect::<Vec<_>>().is_empty() {
            continue;
        }

        rez.insert(x.to_string());
    }
   // println!("{:?}", rez.iter().sorted_by( |x, y|  x.to_lowercase().cmp(&y.to_lowercase())).collect_vec());
        rez.iter().sorted_by( |x, y|  x.cmp(&y)).map( |x| x.to_string()).collect_vec()
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn examples() {
        assert_eq!(in_array(
            &["xyz", "live", "strong"],
            &["lively", "alive", "harp", "sharp", "armstrong"],
        ), ["live", "strong"]);

        assert_eq!(in_array(
            &["live", "strong", "arp"],
            &["lively", "alive", "harp", "sharp", "armstrong"],
        ), ["arp", "live", "strong"]);

        assert_eq!(in_array(
            &["tarp", "mice", "bull"],
            &["lively", "alive", "harp", "sharp", "armstrong"],
        ), [] as [&str; 0]);

        assert_eq!(in_array(
            &["live", "strong", "arp", "arp"],
            &["lively", "alive", "harp", "sharp", "armstrong"],
        ), ["arp", "live", "strong"]);
    }
}
/* Which are in?
https://www.codewars.com/kata/550554fd08b86f84fe000a58/train/rust
Given two arrays of strings a1 and a2 return a sorted array r in lexicographical order of the strings of a1 which are substrings of strings of a2.
Example 1:

a1 = ["arp", "live", "strong"]

a2 = ["lively", "alive", "harp", "sharp", "armstrong"]

returns ["arp", "live", "strong"]
Example 2:

a1 = ["tarp", "mice", "bull"]

a2 = ["lively", "alive", "harp", "sharp", "armstrong"]

returns []
Notes:

    Arrays are written in "general" notation. See "Your Test Cases" for examples in your language.
    In Shell bash a1 and a2 are strings. The return is a string where words are separated by commas.
    Beware: In some languages r must be without duplicates.


*/