

fn bingo<S: AsRef<str>>(ticket: &[(S, u8)], win: usize) -> &'static str {
    let  mut  result = 0 as u8;
    for x in ticket {
      let rez =   (*x).0.as_ref().chars().map( |c| c as  u8 == (*x).1).filter( |e| *e ==true).collect::<Vec<bool>>();
        println!("{:?}", rez);
        result = result  + rez.len() as u8;

    }
    match result >= win as u8 {
        true => "Winner!",
        _ => "Loser!"

    }

}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn basic() {
        assert_eq!(bingo(&[("ABC", 65), ("HGR", 74), ("BYHT", 74)], 2), "Loser!");
        assert_eq!(bingo(&[("ABC", 65), ("HGR", 74), ("BYHT", 74)], 1), "Winner!");
       assert_eq!(bingo(&[("HGTYRE", 74), ("BE", 66), ("JKTY", 74)], 3), "Loser!");
    }
}


/*Time to win the lottery!

Given a lottery ticket (ticket), represented by an array of 2-value arrays, you must find out if you've won the jackpot.

Example ticket:

[ ( "ABC", 65 ), ( "HGR", 74 ), ( "BYHT", 74 ) ]
To do this, you must first count the 'mini-wins' on your ticket. Each subarray has both a string and a number within it. If the character code of any of the characters in the string matches the number, you get a mini win. Note you can only have one mini win per sub array.
Once you have counted all of your mini wins, compare that number to the other input provided (win). If your total is more than or equal to (win), return 'Winner!'. Else return 'Loser!'.
All inputs will be in the correct format. Strings on tickets are not always the same length.
https://www.codewars.com/kata/57f625992f4d53c24200070e/train/rust
*/