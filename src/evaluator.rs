use crate::ast::Program;
use crate::object::Object;

pub enum EvalError {
    UnknownError,
}

pub fn eval(p: Program) -> Result<Object, EvalError> {
    println!("{}", p);
    Err(EvalError::UnknownError)
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn eval_integer_expression_test() {
        panic!();
    }
}