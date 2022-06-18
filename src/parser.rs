use debug_print::{debug_print as dprint, debug_println as dprintln};
use phf::phf_map;
use std::f64;

use crate::errors::*;
use crate::helpers::*;
use crate::math;

static OPERATORS: phf::Map<char, usize> = phf_map! {
    '+' => 0,
    '-' => 0,
    '*' => 1,
    '/' => 1,
    '^' => 2,
    '!' => 3,
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
    "logab" => 2,
    "sqrt" => 1,
    "nroot" => 2,
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

    let first_char = expr.chars().next().unwrap();
    let last_char = expr.chars().last().unwrap();

    if first_char == '"' && last_char == '"' {
        let inner = &expr[1..expr.len() - 1];
        dprintln!("unwrap into {}", inner);
        return parse(inner, full_expr, offset + 1);
    } else if first_char == '(' {
        if let Some(i) = find_closing_parenthesis(expr) {
            if i == expr.len() - 1 {
                let inner = &expr[1..expr.len() - 1];
                dprintln!("unwrap into {}", inner);
                return parse(inner, full_expr, offset + 1);
            }
        } else {
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
            && ((OPERATORS.get(&c).unwrap() <= &split_precedence && split_char != ' ')
                || split_char == ' ')
        {
            split_char = c;
            split_pos = i;
            split_precedence = *OPERATORS.get(&c).unwrap();
        }
    }

    if split_char == ' ' {
        if let Ok(val) = expr.parse::<f64>() {
            dprintln!("float");
            Ok(val)
        } else if CONSTANTS.contains_key(expr) {
            dprintln!("math constant");
            return Ok(*CONSTANTS.get(expr).unwrap());
        } else if let Some((name, args, pos)) = is_function_call(expr) {
            dprintln!("function call: {} {}", name, args);

            if let Some(expected_arg_num) = FUNCTIONS.get(name) {
                let arg_num = count_args(args);

                if arg_num != *expected_arg_num {
                    return Err(SyntaxError::new(
                            expr.to_owned(),
                            full_expr.to_owned(),
                            format!("incorrect number of arguments passed, function {} takes {} parameters but {} {} passed", name, expected_arg_num, arg_num, if arg_num == 1 {"was"} else {"were"}),
                            offset + expr.len() - 1,
                        ));
                }

                match arg_num {
                    1 => match name {
                        "cos" => Ok(parse(args, full_expr, pos + 1)?.cos()),
                        "sin" => Ok(parse(args, full_expr, pos + 1)?.sin()),
                        "tan" => Ok(parse(args, full_expr, pos + 1)?.tan()),
                        "acos" => Ok(parse(args, full_expr, pos + 1)?.acos()),
                        "asin" => Ok(parse(args, full_expr, pos + 1)?.asin()),
                        "atan" => Ok(parse(args, full_expr, pos + 1)?.atan()),
                        "ln" => Ok(parse(args, full_expr, pos + 1)?.ln()),
                        "sqrt" => Ok(parse(args, full_expr, pos + 1)?.sqrt()),
                        _ => unreachable!(),
                    },
                    2 => {
                        let split = find_nth_comma(args, 1).unwrap();
                        let first = parse(&args[..split], full_expr, pos + 1)?;
                        let second = parse(&args[split + 1..], full_expr, pos + split + 2)?;

                        match name {
                            "logab" => Ok(second.ln() / first.ln()), // log_a(b) = ln b / ln a
                            "nroot" => Ok(f64::powf(second, 1. / first)),
                            _ => unreachable!(),
                        }
                    }
                    _ => unreachable!(),
                }
            } else {
                return Err(SyntaxError::new(
                    expr.to_owned(),
                    full_expr.to_owned(),
                    format!("unkown function name \"{}\"", name),
                    offset + expr.len(),
                ));
            }
        } else if last_char == ')' {
            Err(SyntaxError::new(
                expr.to_owned(),
                full_expr.to_owned(),
                "missing opening parenthesis".to_owned(),
                offset + expr.len(),
            ))
        } else {
            Err(SyntaxError::new(
                expr.to_owned(),
                full_expr.to_owned(),
                "unkown token".to_owned(),
                offset + expr.len(),
            ))
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
            if split_char == '!' {
                Err(SyntaxError::new(
                    expr.to_owned(),
                    full_expr.to_owned(),
                    "unexpected token after operator \"!\"".to_owned(),
                    offset + expr.len(),
                ))
            } else {
                parse(right, full_expr, offset + split_pos + 1)
            }
        }?;

        match split_char {
            '+' => Ok(left + right),
            '-' => Ok(left - right),
            '*' => Ok(left * right),
            '/' => Ok(left / right),
            '^' => Ok(if (left + right).abs() < f64::EPSILON {
                f64::NAN
            } else {
                f64::powf(left, right)
            }),
            '!' => Ok(math::fact(left)),
            _ => unreachable!(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_unkown_token() {
        assert_parse_error!("error", "error while parsing token \"error\" in expression \"error\": unkown token, error <-- HERE");
    }

    #[test]
    fn parse_float_simple() {
        assert_parse_result_float!("1", 1.0);
        assert_parse_result_float!("-5", -5.0);
    }

    #[test]
    fn parse_float_decimals() {
        assert_parse_result_float!("1.", 1.0);
        assert_parse_result_float!(".1", 0.1);
        assert_parse_result_float!("1.1", 1.1);
    }

    #[test]
    fn parse_float_nan() {
        assert_parse_result_is!("nan", is_nan);
    }

    #[test]
    fn parse_float_inf() {
        assert_parse_result_is!("inf", is_infinite);
    }

    #[test]
    fn parse_operator_sum() {
        assert_parse_result_float!("5+3", 8.);
    }

    #[test]
    fn parse_operator_difference() {
        assert_parse_result_float!("1-1", 0.);
    }

    #[test]
    fn parse_operator_product() {
        assert_parse_result_float!("2*4", 8.);
    }

    #[test]
    fn parse_operator_quotient() {
        assert_parse_result_float!("10/2", 5.);
    }

    #[test]
    fn parse_operator_exponent() {
        assert_parse_result_float!("2^2", 4.);
    }

    #[test]
    fn parse_operator_factorial() {
        assert_parse_result_is!("(-1)!", is_nan);
        assert_parse_result_float!("0!", 1.);
        assert_parse_result_float!("1!", 1.);
        assert_parse_result_float!("2!", 2.);
        assert_parse_result_float!("3!", 6.);
        assert_parse_result_float!("4!", 24.);
    }

    #[test]
    fn parse_operator_precedence() {
        assert_parse_result_float!("1+2*3", 7.);
        assert_parse_result_float!("2*3-1", 5.);
        assert_parse_result_float!("1+4/2", 3.);
        assert_parse_result_float!("1^2/4", 0.25);
    }

    #[test]
    fn parse_operator_expected_token() {
        assert_parse_error!("1*", "error while parsing token \"1*\" in expression \"1*\": expected token after operator *, 1* <-- HERE");
        assert_parse_error!("*1", "error while parsing token \"*1\" in expression \"*1\": expected token before operator *, *1 <-- HERE");
    }

    #[test]
    fn parse_operator_unexpected_token() {
        assert_parse_error!("1!1", "error while parsing token \"1!1\" in expression \"1!1\": unexpected token after operator \"!\", 1!1 <-- HERE");
    }

    #[test]
    fn parse_implicit_operators() {
        assert_parse_result_float!("2*-2", -4.);
        assert_parse_result_float!("1--1", 2.);
        assert_parse_result_float!("-1*-1", 1.);
    }

    #[test]
    fn parse_unwrap_quotes() {
        assert_parse_result_float!("\"10-10\"", 0.);
    }

    #[test]
    fn parse_unwrap_parenthesis_simple() {
        assert_parse_result_float!("(1+1)", 2.);
    }

    #[test]
    fn parse_unwrap_parenthesis_nested() {
        assert_parse_result_float!("(((1)+1)+(1+(1)))", 4.);
        assert_parse_result_float!("(1+(1+(1+(1+(1+(1+(1)))))))", 7.);
    }

    #[test]
    fn parse_unwrap_parenthesis_missing_closing() {
        assert_parse_error!("(1+1", "error while parsing token \"(1+1\" in expression \"(1+1\": missing closing parenthesis, (1+1 <-- HERE");
        assert_parse_error!("(1+(1)", "error while parsing token \"(1+(1)\" in expression \"(1+(1)\": missing closing parenthesis, (1+(1) <-- HERE");
    }

    #[test]
    fn parse_unwrap_parenthesis_missing_opening() {
        assert_parse_error!("1+1)", "error while parsing token \"1)\" in expression \"1+1)\": missing opening parenthesis, 1+1) <-- HERE");
        assert_parse_error!("(1+1))", "error while parsing token \"(1+1))\" in expression \"(1+1))\": missing opening parenthesis, (1+1)) <-- HERE");
    }

    #[test]
    fn parse_function_single_cos() {
        assert_parse_result_float!("cos(0)", 1.0);
        assert_parse_result_float!("cos(pi)", -1.0);
    }

    #[test]
    fn parse_function_single_sin() {
        assert_parse_result_float!("sin(0)", 0.0);
        assert_parse_result_float!("sin(pi)", 0.0);
    }

    #[test]
    fn parse_function_single_tan() {
        assert_parse_result_float!("tan(0)", 0.0);
    }

    #[test]
    fn parse_function_single_acos() {
        assert_parse_result_float!("acos(0)", f64::consts::FRAC_PI_2);
    }

    #[test]
    fn parse_function_single_asin() {
        assert_parse_result_float!("asin(1)", f64::consts::FRAC_PI_2);
    }

    #[test]
    fn parse_function_single_atan() {
        assert_parse_result_float!("atan(1)", f64::consts::FRAC_PI_4);
    }

    #[test]
    fn parse_function_single_ln() {
        assert_parse_result_is!("ln(0)", is_infinite);
        assert_parse_result_float!("ln(1)", 0.0);
        assert_parse_result_float!("ln(e)", 1.0);
    }

    #[test]
    fn parse_function_single_sqrt() {
        assert_parse_result_float!("sqrt(9)", 3.);
        assert_parse_result_float!("sqrt(4)", 2.);
        assert_parse_result_float!("sqrt(0)", 0.);
        assert_parse_result_is!("sqrt(-1)", is_nan);
    }

    #[test]
    fn parse_function_two_logab() {
        assert_parse_result_float!("logab(2,16)", 4.);
        assert_parse_result_float!("logab(3,9)", 2.);
        assert_parse_result_is!("logab(1,1)", is_nan);
    }

    #[test]
    fn parse_function_two_nroot() {
        assert_parse_result_float!("nroot(3,8)", 2.);
        assert_parse_result_float!("nroot(4,16)", 2.);
    }

    #[test]
    fn parse_function_unkown_name() {
        assert_parse_error!("test()", "error while parsing token \"test()\" in expression \"test()\": unkown function name \"test\", test() <-- HERE");
    }

    #[test]
    fn parse_function_argument_missmatch() {
        assert_parse_error!("sqrt()", "error while parsing token \"sqrt()\" in expression \"sqrt()\": incorrect number of arguments passed, function sqrt takes 1 parameters but 0 were passed, sqrt( <-- HERE");
        assert_parse_error!("sqrt(1,1)", "error while parsing token \"sqrt(1,1)\" in expression \"sqrt(1,1)\": incorrect number of arguments passed, function sqrt takes 1 parameters but 2 were passed, sqrt(1,1 <-- HERE");
        assert_parse_error!("logab()", "error while parsing token \"logab()\" in expression \"logab()\": incorrect number of arguments passed, function logab takes 2 parameters but 0 were passed, logab( <-- HERE");
        assert_parse_error!("logab(1)", "error while parsing token \"logab(1)\" in expression \"logab(1)\": incorrect number of arguments passed, function logab takes 2 parameters but 1 was passed, logab(1 <-- HERE");
        assert_parse_error!("logab(1,1,1)", "error while parsing token \"logab(1,1,1)\" in expression \"logab(1,1,1)\": incorrect number of arguments passed, function logab takes 2 parameters but 3 were passed, logab(1,1,1 <-- HERE");
    }
}
