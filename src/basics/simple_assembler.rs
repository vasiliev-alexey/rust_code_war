use std::collections::HashMap;


fn simple_assembler(program: Vec<&str>) -> HashMap<String, i64> {


    fn parse(reg_map: &HashMap<String, i64>, s: &str) -> i64 {
        match s.parse::<i64>() {
            Ok(val) => val,
            Err(_) => reg_map[s],
        }
    }

    let instr_list: Vec<Vec<&str>> = program.iter().map(|s| s.split(' ').collect()).collect();
    let mut reg_map = HashMap::new();
    let mut pc = 0usize;
    while pc < program.len() {
        let instr = &instr_list[pc];
        match instr[0] {
            "mov" => {
                reg_map.insert(instr[1].to_owned(), parse(&reg_map, instr[2]));
            }
            "inc" => {
                reg_map.get_mut(instr[1]).map(|v| *v += 1);
            }
            "dec" => {
                reg_map.get_mut(instr[1]).map(|v| *v -= 1);
            }
            "jnz" => {
                if parse(&reg_map, instr[1]) != 0 {
                    pc = pc.wrapping_add(instr[2].parse::<i64>().unwrap() as usize);
                    continue;
                }
            }
            _ => panic!(),
        }
        pc += 1;
    }
    reg_map

}

// Add your tests here.
// See https://doc.rust-lang.org/stable/rust-by-example/testing/unit_testing.html

#[cfg(test)]
mod tests {
    use super::*;

    macro_rules! map {
        ($($key:expr => $value:expr),*) => {{
             let mut map = HashMap::new();
             $(
                 map.insert($key.to_string(), $value);
             )*
             map
        }};
    }

    #[test]
    fn short_tests() {
        let program = vec!["mov a 5", "inc a", "dec a", "dec a", "jnz a -1", "inc a"];
        let expected = map! { "a" => 1 };
        compare_registers(expected, simple_assembler(program));



        let program = vec![
         "mov h 36", "mov s 0",
                "jnz s 2",  "jnz h 3",
                "dec h",    "inc s",
                "dec h",    "inc h",
                "dec s",    "inc s",
                "inc s",    "inc s"
        ];
        let expected = map! {  "h" => 36, "s" => 2};
        compare_registers(expected, simple_assembler(program));

        //
        let program = vec![
            "mov c 12",
            "mov b 0",
            "mov a 200",
            "dec a",
            "inc b",
            "jnz a -2",
            "dec c",
            "mov a b",
            "jnz c -5",
            "jnz 0 1",
            "mov c a",
        ];
        let expected = map! { "a" => 409600, "c" => 409600, "b" => 409600};
        compare_registers(expected, simple_assembler(program));
    }

    fn compare_registers(expected: HashMap<String, i64>, actual: HashMap<String, i64>) {
        let result = expected
            .iter()
            .all(|(key, value)| actual.get(key).map(|v| v == value).unwrap_or(false));
        assert!(
            result,
            "Expected the registers to be like that:\n{:#?}\n\nBut got this:\n{:#?}\n",
            expected, actual
        )
    }
}