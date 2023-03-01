use std::collections::HashMap;

fn number_of_pairs(gloves: &[&str]) -> u32 {

    // use itertools::Itertools;
    // gloves.iter()
    //     .counts()
    //     .values()
    //     .map(|&v| v as u32 / 2)
    //     .sum()

    let mut hs = HashMap::new();
    let  mut  rez = 0;
    for x in gloves {
        *hs.entry(x).or_insert(0) += 1;
    }

    for x in hs{
        println!("{:?}" , x);
        if x.1 / 2 >= 1  {
            println!("{}" , x.1 / 2);
            rez = rez + x.1 / 2;
        }
    }
    rez
}


// Add your tests here.
// See https://doc.rust-lang.org/stable/rust-by-example/testing/unit_testing.html

#[cfg(test)]
mod tests {
    use super::number_of_pairs;


    const ERR_MSG: &str = "\nYour result (left) did not match the expected output (right)";


    fn dotest(arr: &[&str], expected: u32) {
        assert_eq!(number_of_pairs(arr), expected, "{ERR_MSG} with gloves = {arr:?}")
    }

    #[test]
    fn fixed_tests() {
        dotest(&["red","red"], 1);
        dotest(&["red","green","blue"], 0);
        dotest(&["gray","black","purple","purple","gray","black"], 3);
        dotest(&[], 0);
        dotest(&["red","green","blue","blue","red","green","red","red","red"], 4);
    }
}

/*
[Pair of gloves](https://www.codewars.com/kata/58235a167a8cb37e1a0000db/train/rust)
Winter is coming, you must prepare your ski holidays. The objective of this kata is to determine the number of pair of gloves you can constitute from the gloves you have in your drawer.
Given an array describing the color of each glove, return the number of pairs you can constitute, assuming that only gloves of the same color can form pairs.

Examples:
input = ["red", "green", "red", "blue", "blue"]
result = 2 (1 red pair + 1 blue pair)

input = ["red", "red", "red", "red", "red", "red"]
result = 3 (3 red pairs)

*/