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
