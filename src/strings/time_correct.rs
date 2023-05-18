fn time_correct(time_str: &str) -> Option<String> {
    if time_str == "" {
        return   None;
    }
    return Some(time_str.to_string());

    /*
      if (!timestring.match(/\d\d:\d\d:\d\d/ig) ) {
          return null;
      }

      let  [h , m, s] = timestring.split(":").map( e => +e);

      if (h < 24 && m < 60 && s < 60 ) {
          return  timestring;
      }

      if (s >= 60) {

           m = m +Math.floor(s /60);
          s = s%60;
      }
     if (m >= 60) {
         h = h +Math.floor(m /60);
         m = m%60;

     }

      if (h  >= 24 ) {
          h = h%24;
      }
     return  `${h.toString().padStart(2, '0')}:${m.toString().padStart(2, '0')}:${s.toString().padStart(2, '0')}`;
     */
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn empty() {
        assert!(time_correct("").is_none());
    }

    #[test]
    fn invalid_format() {
        assert!(time_correct("001122").is_none());
        assert!(time_correct("00;11;22").is_none());
        assert!(time_correct("00:1c:22").is_none());
    }

    #[test]
    fn corrections() {
        assert_eq!(time_correct("09:10:01"), Some(String::from("09:10:01")));
        assert_eq!(time_correct("11:70:10"), Some(String::from("12:10:10")));
        assert_eq!(time_correct("19:99:09"), Some(String::from("20:39:09")));
        assert_eq!(time_correct("19:99:99"), Some(String::from("20:40:39")));
        assert_eq!(time_correct("24:01:01"), Some(String::from("00:01:01")));
        assert_eq!(time_correct("52:01:01"), Some(String::from("04:01:01")));
    }
}

/*
https://www.codewars.com/kata/57873ab5e55533a2890000c7/train/rust
*/