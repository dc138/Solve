use std::fmt;

#[derive(Debug)]
pub struct SyntaxError {
    expression: String,
    full_expression: String,
    message: String,
    position: usize,
}

impl SyntaxError {
    pub fn new(expr: String, full_expr: String, msg: String, pos: usize) -> SyntaxError {
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
