fn counting_valleys(s: &str) -> u32 {
    let mut level = 0;
    let  mut  res = 0;
    for x in s.chars() {
        match x {
            'U' => {level += 1;
                if level == 0 {
                    res += 1;
                }
            },
            'D' => level -= 1,
            _ => level += 0
        }
    }
    res
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn fixed_tests() {
        assert_eq!(counting_valleys("UFFDDFDUDFUFU"), 1);
        assert_eq!(counting_valleys("UFFDDFDUDFUFUUFFDDUFFDDUFFDDUDUDUDUDUDUUUUUUUUU"), 3);
        assert_eq!(counting_valleys("UFFDDFDUDFUFUUFFDDFDUDFUFUUFFDDFDUDFUFUUFFDDFDUDFUFUUFFDDFDUDFUFUUFFDDFDUDFUFU"), 6);
        assert_eq!(counting_valleys("UFFDDFDUDFUFUUFFDDFDUDFUFU"), 2);
        assert_eq!(counting_valleys("UFFDDFDUDFUFUUFFDDFDUDFUFUUFFDDFDUDFUFUUFFDDFDUDFUFU"), 4);
        assert_eq!(counting_valleys("DFFFU"), 1);
        assert_eq!(counting_valleys("UFFFD"), 0);
        assert_eq!(counting_valleys("DFFFD"), 0);
        assert_eq!(counting_valleys("UFFFU"), 0);
    }
}
