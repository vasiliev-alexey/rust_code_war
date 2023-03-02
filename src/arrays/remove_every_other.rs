fn remove_every_other(arr: &[u8]) -> Vec<u8> {
    arr.iter().enumerate().filter(|(i, v)| (i + 1) % 2 != 0).map(|x| *x.1).collect::<Vec<u8>>()
    //  arr.iter().step_by(2).copied().collect()
}


#[cfg(test)]
mod tests {
    use super::remove_every_other;

    #[test]
    fn sample_test() {
        assert_eq!(remove_every_other(&[1, 2, 3, 4, 5, 6, 7, 8, 9, 10]), &[1, 3, 5, 7, 9]);
    }
}


/* Removing Elements
Take an array and remove every second element from the array. Always keep the first element and start removing with the next element.

Example:
["Keep", "Remove", "Keep", "Remove", "Keep", ...] --> ["Keep", "Keep", "Keep", ...]

None of the arrays will be empty, so you don't have to worry about that!


*/