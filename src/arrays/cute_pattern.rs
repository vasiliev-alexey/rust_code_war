fn cute_pattern(tiles: &str) -> bool {

    let lines = tiles.split("\n").filter( |e| e.len() > 0).map(|s| s.split("").filter( |e| e.len() > 0).collect::<Vec<_>>()).collect::<Vec<_>>();
    let size = lines.len() - 1;

    println!("{:?} |{}|", lines , tiles);

    for _ in 0..(size) {
        for i in 0..(size) {
            for j in 0..(size) {
                let a = lines[i][j].to_string();
                let b = lines[i][j + 1].to_string();
                let c = lines[i + 1][j].to_string();
                let d = lines[i + 1][j + 1].to_string();
                if a == b && b == c && c == d {
                    return false;
                }
            }
        }
    }
    true
}


fn run_test(input: &str, result: bool) {
    assert_eq!(cute_pattern(input), result);
}

#[test]
fn test_dots_on_domino_bones() {
    run_test("BWBW\nWBWB\nBWBW\nWBWB", true);
    run_test("WBWB\nBWBW\nWBWB\nBWBW", true);
    run_test("BWBW\nBWBW\nBWBW\nBWBW", true);
    run_test("BBBB\nWWWW\nBBBB\nWWWW", true);
    run_test("BWWB\nWBWW\nWWBW\nBWWB", true);
    run_test("BWWB\nWBWW\nWWBW\nBWWB", true);
    run_test("WWBW\nWBWB\nBWBW\nWBWW", true);
    run_test("WBWW\nBWBW\nWBWB\nWWBW", true);
    run_test("WBWW\nBBBW\nWBWW\nWWWB", true);
    run_test("BWWW\nWWBW\nWBBB\nWWBW", true);
    // //
    run_test("WWWW\nWBBW\nWBBW\nWWWW", false);
    run_test("BBBB\nBWWB\nBWWB\nBBBB", false);
    run_test("BWWB\nBWWB\nWBBW\nWBBW", false);
    run_test("WWWW\nWWWW\nWWWW\nWWWW", false);
    run_test("BBBB\nBBBB\nBBBB\nBBBB", false);
    run_test("BBBB\nBBBB\nWWWW\nWWWW", false);
    run_test("WWWW\nWWWW\nBBBB\nBBBB", false);
    run_test("BBWW\nBBWW\nWWBB\nWWBB", false);
    run_test("WWBB\nWWBB\nBBWW\nBBWW", false);
    run_test("WWWW\nWBBW\nWBBB\nWWBW", false);
    //
    run_test("WWWB\nWBWW\nBBBB\nBWWW\n", false);


}

/* https://www.codewars.com/kata/64087fd72daf09000f60dc26/train/rust
According to the given arrangement of tiles, it is required to determine whether the executed pattern is cute. You need to write a function.

Input data:

A string value is entered into the function by type "BWBW\nBBWB\nWWBB\nBWWW".

B - black tile

W - white tile

\n - included just for line wrapping

Output data:

Return True if the pattern is cute and False otherwise.

Examples:

cute_pattern("BWBW\nBBWB\nWWBB\nBWWW") # should return True
cute_pattern("BBWB\nBBWB\nWWBW\nBBWB") # should return False
If you want to solve our problems, they are here: From Singularity Hub Community

I will gladly accept help for other languages in telegram @fimermaker


*/