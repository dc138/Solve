use debug_print::{debug_print as dprint, debug_println as dprintln};
use phf::phf_map;
use std::f64;

use crate::errors::*;
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
            _ => panic!("reached unexpected code block"),
        };
    }
}
