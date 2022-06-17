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

    None
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

    None
}

#[macro_export]
macro_rules! assert_parse_result_float {
    ($x:expr, $y:expr) => {
        assert!((parse($x, $x, 0).unwrap() - $y).abs() < f64::EPSILON);
    };
}

#[macro_export]
macro_rules! assert_parse_result_is {
    ($x:expr, $y:ident) => {
        assert!((parse($x, $x, 0).unwrap().$y()));
    };
}

#[macro_export]
macro_rules! assert_parse_error {
    ($x:expr, $y:expr) => {
        assert_eq!(format!("{}", parse($x, $x, 0).expect_err("")), $y);
    };
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
