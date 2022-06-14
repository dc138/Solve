use std::env;

mod errors;
mod math;
mod parser;

use parser::*;

fn main() {
    if env::args().len() < 2 {
        println!("Incorrect usage, please do expr <expr> [<expr>...]");
        std::process::exit(1);
    }

    let expr: String = env::args()
        .skip(1)
        .fold(String::new(), |mut partial, current| {
            partial.push_str(&current);
            partial
        })
        .chars()
        .filter(|c: &char| c != &' ')
        .collect();

    match parse(&expr, &expr, 0) {
        Ok(res) => {
            println!("{}", res);
        }
        Err(err) => {
            println!("{}", err);
            std::process::exit(1);
        }
    }
}
