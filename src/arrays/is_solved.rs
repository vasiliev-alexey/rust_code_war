fn is_solved(board: &[&[u8; 3]; 3]) -> i8 {
    fn check_win(p1: &u8, p2: &u8, p3: &u8) -> bool {
        let qe_el = p1 == p2 && p2 == p3;
        qe_el && p1 != &0u8
    }

    for i in 0..3 {
        if check_win(board[i].get(0).unwrap(), board[i].get(1).unwrap(), board[i].get(2).unwrap()) {
            return *board[i].get(0).unwrap() as i8;
        }

        if check_win(board[0].get(i).unwrap(), board[1].get(i).unwrap(), board[2].get(i).unwrap()) {
            return *board[0].get(i).unwrap() as i8;
        }
    }
    if check_win(board[0].get(0).unwrap(), board[1].get(1).unwrap(), board[2].get(2).unwrap()) {
        return *board[0].get(0).unwrap() as i8;
    }

    if check_win(board[0].get(2).unwrap(), board[1].get(1).unwrap(), board[2].get(0).unwrap()) {
        return *board[0].get(2).unwrap() as i8;
    }


    for i in 0..3 {
        if board[i].get(0).unwrap() == &0u8 ||
            board[i].get(1).unwrap() == &0u8 ||
            board[i].get(2).unwrap() == &0u8
        {
            return -1;
        }
    }

    0i8
}


// Add your tests here.
// See https://doc.rust-lang.org/stable/rust-by-example/testing/unit_testing.html

#[cfg(test)]
mod tests {
    use super::is_solved;

    fn dotest(board: &[&[u8; 3]; 3], expected: i8) {
        let actual = is_solved(board);
        assert!(actual == expected, "With board = {board:?}\nExpected {expected} but got {actual}")
    }

    #[test]
    fn fixed_tests() {
        for (board, expected) in [
            ([&[1, 1, 1], &[0, 2, 2], &[0, 0, 0]], 1),
            // ([&[0, 0, 1], &[0, 1, 2], &[2, 1, 0]], -1),
            //
            //
            ([&[2, 1, 2], &[2, 1, 1], &[1, 1, 2]], 1),
            // ([&[2, 1, 2], &[2, 1, 1], &[1, 2, 1]], 0)
        ] {
            dotest(&board, expected);
        }
    }
}
//https://www.codewars.com/kata/525caa5c1bf619d28c000335/train/rust