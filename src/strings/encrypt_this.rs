use itertools::Itertools;

fn encrypt_this(text: &str) -> String {
    let words = text.split_whitespace().collect_vec();
    let mut rez: Vec<String> = Vec::with_capacity(words.len());
    for x in words {
        let mut chars = x.chars().collect_vec();
        let l = x.len();
        if l == 1 {
            rez.push(format!("{}", (*chars.first().unwrap()) as u32));
            continue;
        }
        chars.swap(1, l - 1);
        rez.push(format!("{}{}", (*chars.first().unwrap()) as u32, chars.iter().skip(1).join("")));
    }
    rez.join(" ")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_basic() {
        assert_eq!(encrypt_this(&"A"), "65".to_string());
        assert_eq!(encrypt_this(&"A wise old owl lived in an oak"), "65 119esi 111dl 111lw 108dvei 105n 97n 111ka".to_string());
        assert_eq!(encrypt_this(&"The more he saw the less he spoke"), "84eh 109ero 104e 115wa 116eh 108sse 104e 115eokp".to_string());
        assert_eq!(encrypt_this(&"The less he spoke the more he heard"), "84eh 108sse 104e 115eokp 116eh 109ero 104e 104dare".to_string());
        assert_eq!(encrypt_this(&"Why can we not all be like that wise old bird"), "87yh 99na 119e 110to 97ll 98e 108eki 116tah 119esi 111dl 98dri".to_string());
        assert_eq!(encrypt_this(&"Thank you Piotr for all your help"), "84kanh 121uo 80roti 102ro 97ll 121ruo 104ple".to_string());
    }
}
