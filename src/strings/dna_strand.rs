use itertools::Itertools;

fn dna_strand(dna: &str) -> String {
    // Translate the DNA strand
    dna.chars().map( |c| match c {
        'A' => 'T',
        'T' => 'A',
        'G' => 'C',
        'C' => 'G',
        _ => c
    }).join("")
}

// Rust test example:
// TODO: replace with your own tests (TDD), these are just how-to examples.
// See: https://doc.rust-lang.org/book/testing.html

#[cfg(test)]
mod tests {
    use super::dna_strand;

    fn dotest(s: &str, expected: &str) {
        let actual = dna_strand(s);
        assert!(actual == expected,
                "With dna = \"{s}\"\nExpected \"{expected}\" but got \"{actual}\"")
    }

    #[test]
    fn fixed_tests() {
        dotest("AAAA","TTTT");
        dotest("ATTGC","TAACG");
        dotest("GTAT","CATA");
    }
}