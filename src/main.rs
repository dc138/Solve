use phf::phf_map;
use std::env;

static OPERATORS: phf::Map<char, usize> = phf_map! {
    '+' => 3,
    '-' => 3,
    '*' => 2,
    '/' => 2,
    '^' => 1,
};

fn evaluate(expr: &str) -> f64 {
    let first_char = expr.chars().nth(0).unwrap();
    let last_char = expr.chars().last().unwrap();

    if first_char == '"' && last_char == '"' {
        let inner = &expr[1..expr.len() - 1];
        println!("parse {}: unwrap into {}", expr, inner);
        return evaluate(inner);
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
            println!("parse {}: unwrap into {}", expr, inner);
            return evaluate(inner);
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
            println!("parse {}: float", expr);
            return val;
        }
    } else {
        let left = &expr[..split_pos];
        let right = &expr[split_pos + 1..];

        println!("parse {}: {} {} {}", expr, left, split_char, right);

        let left = if left.is_empty() && (split_char == '+' || split_char == '-') {
            0.
        } else {
            evaluate(left)
        };
        let right = evaluate(right);

        return match split_char {
            '+' => left + right,
            '-' => left - right,
            '*' => left * right,
            '/' => left / right,
            '^' => f64::powf(left, right),
            _ => 0.,
        };
    }

    panic!("Reached end of function without branching");
}

fn main() {
    if env::args().len() < 2 {
        println!("Incorrect usage, please do expr <expr> [<expr>...]");
        std::process::exit(1);
    }

    let expr = env::args().skip(1).fold(String::new(), |partial, current| {
        format!("{}{}", partial, current)
    });

    println!("{}", evaluate(&expr));
}
