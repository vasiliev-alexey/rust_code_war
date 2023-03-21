use std::collections::HashMap;
use once_cell::sync::Lazy;

#[rustfmt::skip]
pub static NATO: Lazy<HashMap<char, &'static str>> = Lazy::new(|| {
    [
        ('A', "Alfa"), ('B', "Bravo"), ('C', "Charlie"), ('D', "Delta"),
        ('E', "Echo"), ('F', "Foxtrot"), ('G', "Golf"), ('H', "Hotel"),
        ('I', "India"), ('J', "Juliett"), ('K', "Kilo"), ('L', "Lima"),
        ('M', "Mike"), ('N', "November"), ('O', "Oscar"), ('P', "Papa"),
        ('Q', "Quebec"), ('R', "Romeo"), ('S', "Sierra"), ('T', "Tango"),
        ('U', "Uniform"), ('V', "Victor"), ('W', "Whiskey"), ('X', "Xray"),
        ('Y', "Yankee"), ('Z', "Zulu"),
    ]
        .iter()
        .copied()
        .collect()
});

fn to_nato(words: &str) -> String {
    //  return  words.split("").filter(w => w.trim().length > 0).map(c =>  alph[c.toLowerCase()] || c     ).join(" ")
   let rez =  words.to_uppercase().chars().filter( |c| *c != ' ').map( | x| {

       if NATO.contains_key(&x) {
           return  NATO[&x].to_string()
       }
       return  x.to_string()

   }).collect::<Vec<_>>();
     rez.join(" ")

}

#[cfg(test)]
mod tests {
    use super::to_nato;

    #[test]
    fn examples() {
        assert_eq!(
            to_nato("If you can read"),
            "India Foxtrot Yankee Oscar Uniform Charlie Alfa November Romeo Echo Alfa Delta"
        );

        assert_eq!(
            to_nato("Did not see that coming",),
            "Delta India Delta November Oscar Tango Sierra Echo Echo Tango Hotel Alfa Tango Charlie Oscar Mike India November Golf"
        );

        assert_eq!(
            to_nato("go for it!"),
            "Golf Oscar Foxtrot Oscar Romeo India Tango !"
        );
    }
}