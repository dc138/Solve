pub fn is_function_call(expr: &str) -> bool {
    for (i, c) in expr.chars().enumerate() {
        if c == '(' && i != 0 {
            let mut par_level = 0;

            for (j, c) in (&expr[i..]).chars().enumerate() {
                par_level += match c {
                    '(' => 1,
                    ')' => -1,
                    _ => 0,
                };

                if par_level == 0 {
                    return j + i == expr.len() - 1;
                }
            }
        } else if !c.is_alphabetic() {
            return false;
        }
    }

    return false;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn helpers_is_function_call() {
        assert!(is_function_call("test(test)"));
        assert!(is_function_call("test()"));
        assert!(is_function_call("test((),())"));
        assert!(!is_function_call("test"));
        assert!(!is_function_call("test(test)a"));
        assert!(!is_function_call("test()a"));
    }
}
