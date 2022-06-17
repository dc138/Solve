pub fn is_function_call(expr: &str) -> Option<(&str, &str, usize)> {
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
                    return if i + j == expr.len() - 1 {
                        Some((&expr[..i], &expr[i + 1..expr.len() - 1], i))
                    } else {
                        None
                    };
                }
            }
        } else if !c.is_alphabetic() {
            return None;
        }
    }

    return None;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn helpers_is_function_call() {
        assert!(is_function_call("test(test)").is_some());
        assert!(is_function_call("test()").is_some());
        assert!(is_function_call("test((),())").is_some());
        assert!(is_function_call("test").is_none());
        assert!(is_function_call("test(test)a").is_none());
        assert!(is_function_call("test()a").is_none());
    }
}
