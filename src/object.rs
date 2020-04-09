use std::fmt;

#[derive(Debug)]
pub enum Object {
    Null,
    Integer(i64),
    Boolean(bool),
    Return(Box<Object>),
}

impl fmt::Display for Object {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Object::Null => write!(f, "null"),
            Object::Integer(value) => write!(f, "{}", value),
            Object::Boolean(value) => write!(f, "{}", value),
            Object::Return(boxed_object) => write!(f, "{}", **boxed_object),
        }
    }
}

impl Object {
    pub fn is_truthy(self) -> bool {
        match self {
            Object::Boolean(value) => value,
            Object::Null => false,
            _ => true,
        }
    }
}
