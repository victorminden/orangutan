use std::fmt;

#[derive(Debug)]
pub enum Object {
    Integer(i64),
}

impl fmt::Display for Object {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Object::Integer(int) => write!(f, "{}", int),
        }
    }
}
