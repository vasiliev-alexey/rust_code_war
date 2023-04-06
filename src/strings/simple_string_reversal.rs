use itertools::Itertools;

fn solve(s: &str) -> String {
    let mut chars = s.chars().filter(|c| !c.is_ascii_whitespace()).collect_vec();
    s.chars().map( |c|  if !c.is_ascii_whitespace() {chars.pop().unwrap()} else {  c }).join("")
}

#[cfg(test)]
mod tests {
    use super::solve;

    #[test]
    fn basic_tests() {
       // assert_eq!(solve(&"codewars"), "srawedoc");
      //  assert_eq!(solve(&"your code"), "edoc ruoy");
        assert_eq!(solve(&"your code rocks"), "skco redo cruoy");
        assert_eq!(solve(&"i love codewars"), "s rawe docevoli");
    }
}