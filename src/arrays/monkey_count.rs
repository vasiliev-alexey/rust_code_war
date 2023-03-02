

fn monkey_count(n: i32) -> Vec<i32> {
    // your code here
    (1..=n).collect()
        //.iter().enumerate().map(  | (i, v) | i as i32).collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        assert_eq!(monkey_count(5), vec![1, 2, 3, 4, 5]);
        assert_eq!(monkey_count(1), vec![1]);
        assert_eq!(monkey_count(0), vec![]);
        assert_eq!(monkey_count(12), vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12]);
    }


}

/*
Count the Monkeys!
You take your son to the forest to see the monkeys. You know that there are a certain number there (n), but your son is too young to just appreciate the full number, he has to start counting them from 1.

As a good parent, you will sit and count with him. Given the number (n), populate an array with all numbers up to and including that number, but excluding zero.

For example(Input --> Output):

10 --> [1, 2, 3, 4, 5, 6, 7, 8, 9, 10]
 1 --> [1]

*/