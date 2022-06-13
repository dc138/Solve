use debug_print::{debug_print as dprint, debug_println as dprintln};
use phf::phf_map;
use std::env;
use std::fmt;

#[derive(Debug)]
struct SyntaxError {
    expression: String,
    full_expression: String,
    message: String,
    position: usize,
}

impl SyntaxError {
    fn new(expr: String, full_expr: String, msg: String, pos: usize) -> SyntaxError {
        SyntaxError {
            expression: expr,
            full_expression: full_expr,
            message: msg,
            position: pos,
        }
    }
}

impl fmt::Display for SyntaxError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "error while parsing token \"{}\" in expression \"{}\": {}, {} <-- HERE",
            self.expression,
            self.full_expression,
            self.message,
            &self.full_expression[..self.position]
        )
    }
}

static OPERATORS: phf::Map<char, usize> = phf_map! {
    '+' => 3,
    '-' => 3,
    '*' => 2,
    '/' => 2,
    '^' => 1,
};

fn evaluate(expr: &str, full_expr: &str, offset: usize) -> Result<f64, SyntaxError> {
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
        return evaluate(inner, full_expr, offset + 1);
    } else if first_char == '(' {
        let mut i: usize = 0;
        let mut par_level: isize = 1;

        'dowhile: loop {
            i += 1;

            par_level += match expr.chars().nth(i).unwrap() {
                '(' => 1,
                ')' => -1,
                _ => 0,
            };

            if par_level == 0 || i > expr.len() {
                break 'dowhile;
            }
        }

        if i == expr.len() - 1 {
            let inner = &expr[1..expr.len() - 1];
            dprintln!("unwrap into {}", inner);
            return evaluate(inner, full_expr, offset + 1);
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
            evaluate(left, full_expr, offset)
        }?;

        let right = if right.is_empty() {
            Err(SyntaxError::new(
                expr.to_owned(),
                full_expr.to_owned(),
                format!("expected token after operator {}", split_char),
                offset + expr.len(),
            ))
        } else {
            evaluate(right, full_expr, offset + split_pos + 1)
        }?;

        return match split_char {
            '+' => Ok(left + right),
            '-' => Ok(left - right),
            '*' => Ok(left * right),
            '/' => Ok(left / right),
            '^' => Ok(f64::powf(left, right)),
            _ => panic!("reached unexpected code block"),
        };
    }

    panic!("reached unexpected code block");
}

fn main() {
    if env::args().len() < 2 {
        println!("Incorrect usage, please do expr <expr> [<expr>...]");
        std::process::exit(1);
    }

    let expr = env::args()
        .skip(1)
        .fold(String::new(), |mut partial, current| {
            partial.push_str(&current);
            partial
        });

    match evaluate(&expr, &expr, 0) {
        Ok(res) => {
            println!("{}", res);
        }
        Err(err) => {
            println!("{}", err);
            std::process::exit(1);
        }
    }
}
