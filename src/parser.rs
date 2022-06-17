use debug_print::{debug_print as dprint, debug_println as dprintln};
use phf::phf_map;
use std::f64;

use crate::errors::*;
use crate::helpers::*;
use crate::math;

static OPERATORS: phf::Map<char, usize> = phf_map! {
    '+' => 3,
    '-' => 3,
    '*' => 2,
    '/' => 2,
    '^' => 1,
    '!' => 0,
};

static CONSTANTS: phf::Map<&str, f64> = phf_map! {
    "pi" => f64::consts::PI,
    "e" => f64::consts::E,
};

static FUNCTIONS: phf::Map<&str, usize> = phf_map! {
    "cos" => 1,
    "sin" => 1,
    "tan" => 1,
    "acos" => 1,
    "asin" => 1,
    "atan" => 1,
    "ln" => 1,
};

pub fn parse(expr: &str, full_expr: &str, offset: usize) -> Result<f64, SyntaxError> {
    dprint!("parse {} with ctx {} at {}: ", expr, full_expr, offset);

    if expr.chars().filter(|c: &char| *c == ' ').count() == expr.len() {
        return Err(SyntaxError::new(
            expr.to_owned(),
            full_expr.to_owned(),
            "trying to parse an empty token".to_owned(),
            offset,
        ));
    }

    let first_char = expr.chars().nth(0).unwrap();
    let last_char = expr.chars().last().unwrap();

    if first_char == '"' && last_char == '"' {
        let inner = &expr[1..expr.len() - 1];
        dprintln!("unwrap into {}", inner);
        return parse(inner, full_expr, offset + 1);
    } else if first_char == '(' {
        let mut i: usize = 0;
        let mut par_level: isize = 0;

        'dowhile: loop {
            par_level += match expr.chars().nth(i).unwrap() {
                '(' => 1,
                ')' => -1,
                _ => 0,
            };

            i += 1;

            if par_level == 0 {
                break 'dowhile;
            } else if i == expr.len() {
                i = 0;
                break 'dowhile;
            }
        }

        if i == expr.len() {
            let inner = &expr[1..expr.len() - 1];
            dprintln!("unwrap into {}", inner);
            return parse(inner, full_expr, offset + 1);
        } else if i == 0 {
            return Err(SyntaxError::new(
                expr.to_owned(),
                full_expr.to_owned(),
                "missing closing parenthesis".to_owned(),
                offset + expr.len(),
            ));
        }
    }

    let mut split_pos: usize = 0;
    let mut split_precedence: usize = 0;
    let mut split_char: char = ' ';

    let mut skipping: bool = false;
    let mut par_level: isize = 0;

    for (i, c) in expr.chars().enumerate() {
        if c == '(' && !skipping {
            skipping = true;
        }

        if skipping {
            par_level += match c {
                '(' => 1,
                ')' => -1,
                _ => 0,
            };

            if par_level == 0 {
                skipping = false;
            }

            continue;
        } else if OPERATORS.contains_key(&c)
            && !((c == '+' || c == '-')
                && (i != 0 && OPERATORS.contains_key(&expr.chars().nth(i - 1).unwrap())))
        {
            if (OPERATORS.get(&c).unwrap() >= &split_precedence && split_char != ' ')
                || split_char == ' '
            {
                split_char = c;
                split_pos = i;
                split_precedence = *OPERATORS.get(&c).unwrap();
            }
        }
    }

    if split_char == ' ' {
        if let Ok(val) = expr.parse::<f64>() {
            dprintln!("float");
            return Ok(val);
        } else if CONSTANTS.contains_key(&expr) {
            dprintln!("math constant");
            return Ok(*CONSTANTS.get(&expr).unwrap());
        } else if let Some((name, args, pos)) = is_function_call(expr) {
            dprintln!("function call: {} {}", name, args);

            match FUNCTIONS.get(&name) {
                Some(1) => {
                    return match name {
                        "cos" => Ok(parse(args, full_expr, pos + 1)?.cos()),
                        "sin" => Ok(parse(args, full_expr, pos + 1)?.sin()),
                        "tan" => Ok(parse(args, full_expr, pos + 1)?.tan()),
                        "acos" => Ok(parse(args, full_expr, pos + 1)?.acos()),
                        "asin" => Ok(parse(args, full_expr, pos + 1)?.asin()),
                        "atan" => Ok(parse(args, full_expr, pos + 1)?.atan()),
                        "ln" => Ok(parse(args, full_expr, pos + 1)?.ln()),
                        _ => unreachable!(),
                    }
                }
                Some(_) => {
                    unreachable!();
                }
                None => {
                    return Err(SyntaxError::new(
                        expr.to_owned(),
                        full_expr.to_owned(),
                        format!("unkown function name \"{}\"", name),
                        offset + expr.len(),
                    ));
                }
            }
        } else if last_char == ')' {
            return Err(SyntaxError::new(
                expr.to_owned(),
                full_expr.to_owned(),
                "missing opening parenthesis".to_owned(),
                offset + expr.len(),
            ));
        } else {
            return Err(SyntaxError::new(
                expr.to_owned(),
                full_expr.to_owned(),
                "unkown token".to_owned(),
                offset + expr.len(),
            ));
        }
    } else {
        let left = &expr[..split_pos];
        let right = &expr[split_pos + 1..];

        dprintln!("{} {} {}", left, split_char, right);

        let left = if left.is_empty() {
            if split_char == '+' || split_char == '-' {
                Ok(0.)
            } else {
                Err(SyntaxError::new(
                    expr.to_owned(),
                    full_expr.to_owned(),
                    format!("expected token before operator {}", split_char),
                    offset + expr.len(),
                ))
            }
        } else {
            parse(left, full_expr, offset)
        }?;

        let right = if right.is_empty() {
            if split_char == '!' {
                Ok(0.)
            } else {
                Err(SyntaxError::new(
                    expr.to_owned(),
                    full_expr.to_owned(),
                    format!("expected token after operator {}", split_char),
                    offset + expr.len(),
                ))
            }
        } else {
            parse(right, full_expr, offset + split_pos + 1)
        }?;

        return match split_char {
            '+' => Ok(left + right),
            '-' => Ok(left - right),
            '*' => Ok(left * right),
            '/' => Ok(left / right),
            '^' => Ok(if (left + right) < f64::EPSILON {
                f64::NAN
            } else {
                f64::powf(left, right)
            }),
            '!' => Ok(math::fact(left)),
            _ => unreachable!(),
        };
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_unkown_token() {
        let expr: &str = "error";
        assert_eq!(format!("{}", parse(expr, expr, 0).expect_err("")), "error while parsing token \"error\" in expression \"error\": unkown token, error <-- HERE");
    }

    #[test]
    fn parse_float_simple() {
        let expr: &str = "1";
        assert_eq!(parse(expr, expr, 0).unwrap(), 1.);

        let expr: &str = "-5";
        assert_eq!(parse(expr, expr, 0).unwrap(), -5.);
    }

    #[test]
    fn parse_float_decimals() {
        let expr: &str = "1.";
        assert_eq!(parse(expr, expr, 0).unwrap(), 1.0);

        let expr: &str = ".1";
        assert_eq!(parse(expr, expr, 0).unwrap(), 0.1);

        let expr: &str = "1.1";
        assert_eq!(parse(expr, expr, 0).unwrap(), 1.1);
    }

    #[test]
    fn parse_float_nan() {
        let expr: &str = "nan";
        assert!(parse(expr, expr, 0).unwrap().is_nan());
    }

    #[test]
    fn parse_float_inf() {
        let expr: &str = "inf";
        assert!(parse(expr, expr, 0).unwrap().is_infinite());
    }

    #[test]
    fn parse_operator_sum() {
        let expr: &str = "5+3";
        assert_eq!(parse(expr, expr, 0).unwrap(), 8.);
    }

    #[test]
    fn parse_operator_difference() {
        let expr: &str = "1-1";
        assert_eq!(parse(expr, expr, 0).unwrap(), 0.);
    }

    #[test]
    fn parse_operator_product() {
        let expr: &str = "2*4";
        assert_eq!(parse(expr, expr, 0).unwrap(), 8.);
    }

    #[test]
    fn parse_operator_quotient() {
        let expr: &str = "10/2";
        assert_eq!(parse(expr, expr, 0).unwrap(), 5.);
    }

    #[test]
    fn parse_operator_exponent() {
        let expr: &str = "2^2";
        assert_eq!(parse(expr, expr, 0).unwrap(), 4.);
    }

    #[test]
    fn parse_operator_precedence() {
        let expr: &str = "1+2*3";
        assert_eq!(parse(expr, expr, 0).unwrap(), 7.);

        let expr: &str = "2*3-1";
        assert_eq!(parse(expr, expr, 0).unwrap(), 5.);

        let expr: &str = "1+4/2";
        assert_eq!(parse(expr, expr, 0).unwrap(), 3.);

        let expr: &str = "1^2/4";
        assert_eq!(parse(expr, expr, 0).unwrap(), 0.25);
    }

    #[test]
    fn parse_implicit_operators() {
        let expr: &str = "2*-2";
        assert_eq!(parse(expr, expr, 0).unwrap(), -4.);

        let expr: &str = "1--1";
        assert_eq!(parse(expr, expr, 0).unwrap(), 2.);

        let expr: &str = "-1*-1";
        assert_eq!(parse(expr, expr, 0).unwrap(), 1.);
    }

    #[test]
    fn parse_unwrap_quotes() {
        let expr: &str = "\"10-10\"";
        assert_eq!(parse(expr, expr, 0).unwrap(), 0.);
    }
    #[test]
    fn parse_unwrap_parenthesis_simple() {
        let expr: &str = "(1+1)";
        assert_eq!(parse(expr, expr, 0).unwrap(), 2.);
    }
    #[test]
    fn parse_unwrap_parenthesis_nested() {
        let expr: &str = "(((1)+1)+(1+(1)))";
        assert_eq!(parse(expr, expr, 0).unwrap(), 4.);

        let expr: &str = "(1+(1+(1+(1+(1+(1+(1)))))))";
        assert_eq!(parse(expr, expr, 0).unwrap(), 7.);
    }

    #[test]
    fn parse_unwrap_parenthesis_missing_closing() {
        let expr: &str = "(1+1";
        assert_eq!(format!("{}", parse(expr, expr, 0).expect_err("")), "error while parsing token \"(1+1\" in expression \"(1+1\": missing closing parenthesis, (1+1 <-- HERE");

        let expr: &str = "(1+(1)";
        assert_eq!(format!("{}", parse(expr, expr, 0).expect_err("")), "error while parsing token \"(1+(1)\" in expression \"(1+(1)\": missing closing parenthesis, (1+(1) <-- HERE");
    }

    #[test]
    fn parse_unwrap_parenthesis_missing_opening() {
        let expr: &str = "1+1)";
        assert_eq!(format!("{}", parse(expr, expr, 0).expect_err("")), "error while parsing token \"1)\" in expression \"1+1)\": missing opening parenthesis, 1+1) <-- HERE");

        let expr: &str = "(1+1))";
        assert_eq!(format!("{}", parse(expr, expr, 0).expect_err("")), "error while parsing token \"(1+1))\" in expression \"(1+1))\": missing opening parenthesis, (1+1)) <-- HERE");
    }

    #[test]
    fn parse_function_single_cos() {
        let expr: &str = "cos(0)";
        assert_eq!(parse(expr, expr, 0).unwrap(), 1.0);

        let expr: &str = "cos(pi)";
        assert_eq!(parse(expr, expr, 0).unwrap(), -1.0);
    }

    #[test]
    fn parse_function_single_sin() {
        let expr: &str = "sin(0)";
        assert!(parse(expr, expr, 0).unwrap().abs() < f64::EPSILON);

        let expr: &str = "sin(pi)";
        assert!(parse(expr, expr, 0).unwrap().abs() < f64::EPSILON);
    }

    #[test]
    fn parse_function_single_tan() {
        let expr: &str = "tan(0)";
        assert_eq!(parse(expr, expr, 0).unwrap(), 0.0);
    }

    #[test]
    fn parse_function_single_acos() {
        let expr: &str = "acos(0)";
        assert_eq!(parse(expr, expr, 0).unwrap(), f64::consts::FRAC_PI_2);
    }

    #[test]
    fn parse_function_single_asin() {
        let expr: &str = "asin(1)";
        assert_eq!(parse(expr, expr, 0).unwrap(), f64::consts::FRAC_PI_2);
    }

    #[test]
    fn parse_function_single_atan() {
        let expr: &str = "atan(1)";
        assert_eq!(parse(expr, expr, 0).unwrap(), f64::consts::FRAC_PI_4);
    }

    #[test]
    fn parse_function_single_ln() {
        let expr: &str = "ln(0)";
        assert!(parse(expr, expr, 0).unwrap().is_infinite());

        let expr: &str = "ln(1)";
        assert_eq!(parse(expr, expr, 0).unwrap(), 0.0);

        let expr: &str = "ln(e)";
        assert_eq!(parse(expr, expr, 0).unwrap(), 1.0);
    }

    #[test]
    fn parse_function_unkown_name() {
        let expr: &str = "test()";
        assert_eq!(format!("{}", parse(expr, expr, 0).expect_err("")), "error while parsing token \"test()\" in expression \"test()\": unkown function name \"test\", test() <-- HERE");
    }
}
