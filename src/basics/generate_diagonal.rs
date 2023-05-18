fn generate_diagonal(base: u8, l: usize) -> Vec<u64> {
println!("{base} {l}");

    if l == 1 {
        return vec![];
    }
    let mut res: Vec<u64> =Vec::with_capacity(l);
    res.push(1u64);

    for k in  1..l  {
        let last = res.last().unwrap();
        let val = last * (base as u64 +k as u64) / (k as u64);
        res.push(val);
    }
    res
}
//https://www.codewars.com/kata/576b072359b1161a7b000a17/train/rust
// Add your tests here.
// See https://doc.rust-lang.org/stable/rust-by-example/testing/unit_testing.html

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sample_tests() {
        assert_eq!(generate_diagonal(2, 5), vec![1, 3, 6, 10, 15]);
        assert_eq!(generate_diagonal(1, 10), vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10]);
        assert_eq!(generate_diagonal(3, 7), vec![1, 4, 10, 20, 35, 56, 84]);
    }
}