
// MORSE_CODE is `HashMap<String, String>`. e.g. ".-" -> "A".

use std::collections::HashMap;
use itertools::Itertools;

pub fn decode_bits(encoded: &str) -> String {
    // Trim excess zeros at the start and end
    let encoded = encoded.trim_matches('0');

    // Get the length of a time unit by finding the shortest sequence of zeros or ones,
    // this will represent a time unit of one which equals a dot
    let rate = {
        let rate_ones = encoded
            .split("0")
            .filter_map(|ones| (!ones.is_empty()).then(|| ones.len()))
            .min()
            .unwrap_or(usize::MAX);
        let rate_zeros = encoded
            .split("1")
            .filter_map(|zeros| (!zeros.is_empty()).then(|| zeros.len()))
            .min()
            .unwrap_or(usize::MAX);

     //   println!("{}  | {}", rate_ones , rate_zeros);

        rate_zeros.min(rate_ones)
    };

    // Parse the encoded message
    encoded
        .chars() // Iterate through the characters
        .step_by(rate) // Only parse every n-th code
        .collect::<String>() // Collect it into a string
        // Begin converting from 1/0 to dot/dash
        .replace("111", "-") // Dash
        .replace("1", ".") // Dot
        .replace("0000000", "   ") // Word seperator
        .replace("000", " ") // Letter seperator
        .replace("0", "") // Dot/Dash seperator
}


fn decode_morse(encoded: &str) -> String {
    let morse_code: HashMap<String, String> = HashMap::from([
        (String::from(".-"), String::from("A")),
        (String::from("-..."), String::from("B")),
        (String::from("-.-."), String::from("C")),
        (String::from("-.."), String::from("D")),
        (String::from("."), String::from("E")),
        (String::from("..-."), String::from("F")),
        (String::from("--."), String::from("G")),
        (String::from("...."), String::from("H")),
        (String::from(".."), String::from("I")),
        (String::from(".---"), String::from("J")),
        (String::from("-.-"), String::from("K")),
        (String::from(".-.."), String::from("L")),
        (String::from("--"), String::from("M")),
        (String::from("-."), String::from("N")),
        (String::from("---"), String::from("O")),
        (String::from(".--."), String::from("P")),
        (String::from("--.-"), String::from("Q")),
        (String::from(".-."), String::from("R")),
        (String::from("..."), String::from("S")),
        (String::from("-"), String::from("T")),
        (String::from("..-"), String::from("U")),
        (String::from("...-"), String::from("V")),
        (String::from(".--"), String::from("W")),
        (String::from("-..-"), String::from("X")),
        (String::from("-.--"), String::from("Y")),
        (String::from("--.."), String::from("Z")),
    ]);

    let words = encoded.trim().split("   ").map( |x| x).collect_vec();
    let mut rez = Vec::new();
    for x in words.iter() {
       let w =  x.split_whitespace().map( |c| morse_code.get(c).unwrap()).join("");
        rez.push(w);
    }

    // println!("{:?}", words);

    rez.join(" ").to_string()
}



//





// Add your tests here.
// See https://doc.rust-lang.org/stable/rust-by-example/testing/unit_testing.html

#[test]
fn examples() {
    assert_eq!(decode_morse(&decode_bits("1100110011001100000011000000111111001100111111001111110000000000000011001111110011111100111111000000110011001111110000001111110011001100000011")), "HEY JUDE".to_string());
}
