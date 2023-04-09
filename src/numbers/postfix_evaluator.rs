use itertools::Itertools;

fn postfix_evaluator(expr: &str) -> i64 {
    let expressions = expr.split_whitespace().collect_vec();
    let operators = ["+", "-", "*", "/", "(", ")", "%", "?"];
    let mut arg_stack: Vec<String> = vec![];
    let mut x ;
    let mut y ;

    for exp in expressions {
        if operators.contains(&exp) {
            x = arg_stack.pop().unwrap().parse::<i64>().unwrap();
            y = arg_stack.pop().unwrap().parse::<i64>().unwrap();

            match exp {
                "%" => &arg_stack.push((y % x).to_string()),
                "*" => &arg_stack.push((y * x).to_string()),
                "/" => &arg_stack.push((y / x).to_string()),
                "+" => &arg_stack.push((y + x).to_string()),
                "-" => &arg_stack.push((y - x).to_string()),
                _ => unreachable!()
            };
        } else {
            arg_stack.push(exp.to_string());
        }
    }
   return  arg_stack.pop().unwrap().parse::<i64>().unwrap()
}

// https://www.codewars.com/kata/577e9095d648a15b800000d4/train/rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn basic() {
        // Simple addition
        assert_eq!(postfix_evaluator("2 3 +"), 5);

        // Addition with negative numbers
        assert_eq!(postfix_evaluator("2 -3 +"), -1);

        // Constant numbers
        assert_eq!(postfix_evaluator("1"), 1);
        assert_eq!(postfix_evaluator("-1"), -1);

        // Complex expressions
        assert_eq!(postfix_evaluator("2 3 9 4 / + *"), 10);
        assert_eq!(postfix_evaluator("3 4 9 / *"), 0);
        assert_eq!(postfix_evaluator("4 8 + 6 5 - * 3 2 - 2 2 + * /"), 3);

        // Multi-digit
        assert_eq!(postfix_evaluator("21 21 +"), 42);
    }
}
