use itertools::Itertools;

fn two_sort(arr: &[&str]) -> String {

    let mut vec = arr.to_vec();
    vec.sort();
    println!("{:?}" , vec.get(0).unwrap());
     vec.get(0).unwrap().chars().join("***").to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sample_test_cases() {
        assert_eq!(two_sort(&["bitcoin", "take", "over", "the", "world", "maybe", "who", "knows", "perhaps"]), "b***i***t***c***o***i***n");
        assert_eq!(two_sort(&["turns", "out", "random", "test", "cases", "are", "easier", "than", "writing", "out", "basic", "ones"]), "a***r***e");
    }
}

//https://www.codewars.com/kata/57cfdf34902f6ba3d300001e/train/rust