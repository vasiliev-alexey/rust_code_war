fn format_duration(seconds: u64) -> String {
    // Complete this function

    let s = seconds % 60;
    let m = seconds % 3600 / 60;
    let h = seconds % 86_400 / 3_600;
    let d = seconds % 31_536_000 / 86_400;
    let y = seconds / 31_536_000;

    let mut rez = vec![];


    if y > 0 {
        let str_value = format!("{} year{}", y, if y > 1 { "s" } else { "" });
        rez.push(str_value);
    };
    if d > 0 {
        let str_value = format!("{} day{}", d, if d > 1 { "s" } else { "" });
        rez.push(str_value);
    };
    if h > 0 {
        let str_value = format!("{} hour{}", h, if h > 1 { "s" } else { "" });
        rez.push(str_value);
    };
    if m > 0 {
        let str_value = format!("{} minute{}", m, if m > 1 { "s" } else { "" });
        rez.push(str_value);
    };
    if s > 0 {
        let str_value = format!("{} second{}", s, if s > 1 { "s" } else { "" });
        rez.push(str_value);
    };

    let mut rrr = String::new();
    for x in rez.clone() {
        if rrr.len() > 0 && &x == rez.last().unwrap() {
            rrr.push_str(" and ")  ;
        } else {
            if rrr.len() > 0 {
                rrr.push_str(", ");
            }

        }
        rrr.push_str( &x);
    }
    println!("{:?}", rrr);


    if seconds > 0 {rrr} else { "now".to_string() }
}


#[cfg(test)]
mod tests {
    use super::format_duration;

    #[test]
    fn test_basic() {
        assert_eq!(format_duration(0), "now");
        assert_eq!(format_duration(1), "1 second");
        assert_eq!(format_duration(62), "1 minute and 2 seconds");
        assert_eq!(format_duration(120), "2 minutes");
        assert_eq!(format_duration(3600), "1 hour");
        assert_eq!(format_duration(7200), "2 hours");
        assert_eq!(format_duration(3662), "1 hour, 1 minute and 2 seconds");
        assert_eq!(format_duration(15731080), "182 days, 1 hour, 44 minutes and 40 seconds");
    }
}


/*
      expect(formatDuration(0)).toEqual("now");
        expect(formatDuration(1)).toEqual("1 second");
        expect(formatDuration(2)).toEqual("2 seconds");
        expect(formatDuration(62)).toEqual("1 minute and 2 seconds");
        expect(formatDuration(120)).toEqual("2 minutes");
        expect(formatDuration(7200)).toEqual("2 hours");
        expect(formatDuration(3600)).toEqual("1 hour");
        expect(formatDuration(3662)).toEqual("1 hour, 1 minute and 2 seconds");
        expect(formatDuration(15731080)).toEqual("182 days, 1 hour, 44 minutes and 40 seconds");
*/