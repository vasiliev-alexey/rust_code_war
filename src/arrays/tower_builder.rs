fn tower_builder(n_floors: usize, block_size: (usize, usize)) -> Vec<String> {
    let (width, height) = block_size;

    let mut rez: Vec<String> = Vec::with_capacity(n_floors * height);
    let mut n_blocks = n_floors;
    for i in 0..n_floors {
        n_blocks = n_blocks - 1;
        for _ in 0..height {
            let stars = "*".repeat((i * 2 + 1) * width);
            let spaces = " ".repeat(n_blocks * width);
            let section = format!("{spaces}{stars}{spaces}");
            rez.push(section);
        }
    }
    rez
}

// Add your tests here.
// See https://doc.rust-lang.org/stable/rust-by-example/testing/unit_testing.html
// https://www.codewars.com/kata/57675f3dedc6f728ee000256/train/rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_fixed() {
        assert_eq!(tower_builder(1, (1, 1)), vec!["*"]);
        assert_eq!(tower_builder(3, (4, 2)), vec!["        ****        ", "        ****        ", "    ************    ", "    ************    ", "********************", "********************"]);
    }
}
