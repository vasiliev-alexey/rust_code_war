use std::cmp;

fn longest_slide_down(pyramid: &[Vec<u16>]) -> u16 {
    if pyramid.len() == 1 {
        return pyramid[0][0];
    }
    if pyramid.len() == 2 {
        return pyramid[0][0] + cmp::max(pyramid[1][0], pyramid[1][1]);
    }
    let mut new_pyramid = pyramid.iter().map(|x| x.clone()).collect::<Vec<_>>();
    for i in (0..new_pyramid.len() - 1).rev() {
        for j in (0..new_pyramid[i].len()).rev() {
            new_pyramid[i][j] = new_pyramid[i][j] + cmp::max(new_pyramid[i + 1][j], new_pyramid[i + 1][j + 1]);
        }
    }
    return new_pyramid[0][0];
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_small() {
        let small = vec![
            vec![3],
            vec![7, 4],
            vec![2, 4, 6],
            vec![8, 5, 9, 3],
        ];
        assert_eq!(longest_slide_down(&small), 23, "It should work for small pyramids");
    }

    #[test]
    fn test_medium() {
        let medium = vec![
            vec![75],
            vec![95, 64],
            vec![17, 47, 82],
            vec![18, 35, 87, 10],
            vec![20, 4, 82, 47, 65],
            vec![19, 1, 23, 75, 3, 34],
            vec![88, 2, 77, 73, 7, 63, 67],
            vec![99, 65, 4, 28, 6, 16, 70, 92],
            vec![41, 41, 26, 56, 83, 40, 80, 70, 33],
            vec![41, 48, 72, 33, 47, 32, 37, 16, 94, 29],
            vec![53, 71, 44, 65, 25, 43, 91, 52, 97, 51, 14],
            vec![70, 11, 33, 28, 77, 73, 17, 78, 39, 68, 17, 57],
            vec![91, 71, 52, 38, 17, 14, 91, 43, 58, 50, 27, 29, 48],
            vec![63, 66, 4, 68, 89, 53, 67, 30, 73, 16, 69, 87, 40, 31],
            vec![4, 62, 98, 27, 23, 9, 70, 98, 73, 93, 38, 53, 60, 4, 23],
        ];
        assert_eq!(longest_slide_down(&medium), 1074, "It should work for medium pyramids");
    }
}

/*
Lyrics...
https://www.codewars.com/kata/551f23362ff852e2ab000037/train/rust
Pyramids are amazing! Both in architectural and mathematical sense. If you have a computer, you can mess with pyramids even if you are not in Egypt at the time. For example, let's consider the following problem. Imagine that you have a pyramid built of numbers, like this one here:

   /3/
  \7\ 4
 2 \4\ 6
8 5 \9\ 3
Here comes the task...
Let's say that the 'slide down' is the maximum sum of consecutive numbers from the top to the bottom of the pyramid. As you can see, the longest 'slide down' is 3 + 7 + 4 + 9 = 23

Your task is to write a function that takes a pyramid representation as an argument and returns its largest 'slide down'. For example:

* With the input `[[3], [7, 4], [2, 4, 6], [8, 5, 9, 3]]`
* Your function should return `23`.
*/