#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub enum Object {
    Integer(i32),
    Boolean(bool),
    ReturnValue(Box<Object>),
    Null,
    Error(String),
}

impl Object {
    pub fn inspect(&self) -> String {
        return match self {
            Object::Integer(i) => i.to_string(),
            Object::Boolean(b) => b.to_string(),
            Object::ReturnValue(r) => r.inspect(),
            Object::Error(e) => format!("Error: {}", e),
            Object::Null => "null".to_string(),
        };
    }
}

pub fn get_type(obj: &Object) -> String {
    return match obj {
        Object::Integer(_) => String::from("INTEGER"),
        Object::Boolean(_) => String::from("BOOLEAN"),
        Object::ReturnValue(_) => String::from("RETURN_VALUE"),
        Object::Error(_) => String::from("ERROR"),
        Object::Null => String::from("NULL"),
    };
}
