fn is_valid_ip(ip: &str) -> bool {

    //  return str.split(".").map(e => Number.parseInt(e).toString() == e && Number.parseInt(e) <= 255 && Number.parseInt(e) >= 0).filter(e => e).length === 4;
    let v =   ip.split(".").map(|oct|oct.parse::<i32>().unwrap_or(-1).to_string()).collect::<Vec<_>>();
    println!( "{:?}" , v);
   let    rez =   ip.split(".").map(|oct|
        oct.parse::<i32>().unwrap_or(-1).to_string() == oct &&
            oct.parse::<i32>().unwrap_or(-1) > -1 &&
            oct.parse::<i32>().unwrap_or(-1) < 256
    ).collect::<Vec<bool>>();
    rez.iter().filter( |e|  **e ).collect::<Vec<_>>().len() == 4 &&  rez.len() == 4



}

// Add your tests here.
// See https://doc.rust-lang.org/stable/rust-by-example/testing/unit_testing.html

#[cfg(test)]
mod tests {
    use super::is_valid_ip;

    #[test]
    fn sample_test() {
        assert!(is_valid_ip("0.0.0.0"));
        assert!(is_valid_ip("12.255.56.1"));
        assert!(is_valid_ip("137.255.156.100"));

        assert!(!is_valid_ip(""));
        assert!(!is_valid_ip("abc.def.ghi.jkl"));
        assert!(!is_valid_ip("123.456.789.0"));
        assert!(!is_valid_ip("12.34.56"));
        assert!(!is_valid_ip("01.02.03.04"));
        assert!(!is_valid_ip("256.1.2.3"));
        assert!(!is_valid_ip("1.2.3.4.5"));
        assert!(!is_valid_ip("123,45,67,89"));
        assert!(!is_valid_ip("1e0.1e1.1e2.2e2"));
        assert!(!is_valid_ip(" 1.2.3.4"));
        assert!(!is_valid_ip("1.2.3.4 "));
        assert!(!is_valid_ip("12.34.56.-7"));
        assert!(!is_valid_ip("1.2.3.4\n"));
        assert!(!is_valid_ip("\n1.2.3.4"));
        assert!(!is_valid_ip("1.2.3.4."));
    }
}


/*
Write an algorithm that will identify valid IPv4 addresses in dot-decimal format. IPs should be considered valid if they consist of four octets, with values between 0 and 255, inclusive.

Valid inputs examples:
Examples of valid inputs:
1.2.3.4
123.45.67.89
Invalid input examples:
1.2.3
1.2.3.4.5
123.456.78.90
123.045.067.089
Notes:
Leading zeros (e.g. 01.02.03.04) are considered invalid
Inputs are guaranteed to be a single string

*/