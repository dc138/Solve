pub fn find_closing_parenthesis(expr: &str) -> Option<usize> {
    let mut par_level: isize = 0;

    for (i, c) in expr.chars().enumerate() {
        par_level += match c {
            '(' => 1,
            ')' => -1,
            _ => 0,
        };

        if par_level == 0 {
            return Some(i);
        }
    }

    return None;
}

pub fn is_function_call(expr: &str) -> Option<(&str, &str, usize)> {
    for (i, c) in expr.chars().enumerate() {
        if c == '(' && i != 0 {
            if let Some(j) = find_closing_parenthesis(&expr[i..]) {
                return if i + j == expr.len() - 1 {
                    Some((&expr[..i], &expr[i + 1..expr.len() - 1], i))
                } else {
                    None
                };
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
    fn helpers_find_closing_parenthesis() {
        assert_eq!(find_closing_parenthesis("(test)").unwrap(), 5);
        assert_eq!(find_closing_parenthesis("()").unwrap(), 1);
        assert!(find_closing_parenthesis("(test").is_none());
        assert!(find_closing_parenthesis("(").is_none());
    }

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
