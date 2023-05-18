

fn compute_depth (n: u16) -> u8 {
    let mut arr   = vec![];
    let mut step = 1u8;

    while arr.len() != 10  {

        (step as u16 * (n as u16)).to_string().chars().for_each( |c| {
            if  !arr.contains(&c) {
                arr.push(c)
            }
        });
        step +=1;
    }
    step -1

}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sample_tests() {
        assert_eq!(compute_depth(1), 10);
        assert_eq!(compute_depth(42), 9);
        assert_eq!(compute_depth(8), 12);
        assert_eq!(compute_depth(13), 8);
        assert_eq!(compute_depth(7), 10);
        assert_eq!(compute_depth(25), 36);
    }
}

/* https://www.codewars.com/kata/59b401e24f98a813f9000026/train/rust
The depth of an integer n is defined to be how many multiples of n it is necessary to compute before all 10 digits have appeared at least once in some multiple.

example:

let see n=42

Multiple         value         digits     comment
42*1              42            2,4
42*2              84             8         4 existed
42*3              126           1,6        2 existed
42*4              168            -         all existed
42*5              210            0         2,1 existed
42*6              252            5         2 existed
42*7              294            9         2,4 existed
42*8              336            3         6 existed
42*9              378            7         3,8 existed
Looking at the above table under digits column you can find all the digits from 0 to 9, Hence it required 9 multiples of 42 to get all the digits. So the depth of 42 is 9. Write a function named computeDepth which computes the depth of its integer argument.Only positive numbers greater than zero will be passed as an input.
*/