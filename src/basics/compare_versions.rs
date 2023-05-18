fn compare_versions(version1: &str, version2: &str) -> bool {
    let mut v1 = version1.split(".").map(|x| x.parse::<u16>().unwrap()).collect::<Vec<_>>();
    let v2 = version2.split(".").map(|x| x.parse::<u16>().unwrap()).collect::<Vec<_>>();
    v1.resize(v2.len(), 0);
    v1.iter().zip(&v2).filter(|&(a, b)| {
        a < b
    }).count() == 0
}


// Add your tests here.
// See https://doc.rust-lang.org/stable/rust-by-example/testing/unit_testing.html

#[cfg(test)]
mod tests {
    use super::compare_versions;

    fn dotest(v1: &str, v2: &str, expected: bool) {
        let actual = compare_versions(v1, v2);
        assert!(actual == expected, "With version1 = \"{v1}\", version2 = \"{v2}\"\nExpected {expected} but got {actual}")
    }

    #[test]
    fn fixed_tests() {
        dotest("11", "10", true);
        dotest("11", "11", true);
        dotest("10.4.6", "10.4", true);
        dotest("10.4", "10.4.8", false);
        dotest("10.4", "11", false);
        dotest("10.4.9", "10.5", false);
        dotest("4.3.3", "4.3.3.1", false);
        dotest("10.4.9", "10.5", false);
        dotest("10.4.9", "104.9", false);
        dotest("10.15", "10.12", true);
    }
}

/* https://www.codewars.com/kata/53b138b3b987275b46000115/train/rust

Compare Versions
Karan's company makes software that provides different features based on the version of operating system of the user.

For finding which version is more recent, Karan uses the following method:

function compareVersions (version1, version2) {
  return parseFloat(version1) >= parseFloat(version2);
}
While this function worked for OS versions 10.6, 10.7, 10.8 and 10.9, the Operating system company just released OS version 10.10.

Karan's function fails for the new version:

compareVersions ("10.9", "10.10");       // returns true, while it should return false
Karan now wants to spend some time to write a more robust version comparison function that works for any future version/sub-version updates.

Help Karan write this function. Here are a few sample cases:

compareVersions("11", "10");                    // returns true
compareVersions("11", "11");                    // returns true
compareVersions("10.4.6", "10.4");              // returns true
compareVersions("10.4", "11");                  // returns false
compareVersions("10.4", "10.10");               // returns false
compareVersions("10.4.9", "10.5");              // returns false
It can be assumed that version strings are non empty and only contain numeric literals and the character '.'.


*/