#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub enum Object {
    Integer(i32),
    Boolean(bool),
    Null,
}

impl Object {
    pub fn inspect(&self) -> String {
        return match self {
            Object::Integer(i) => i.to_string(),
            Object::Boolean(b) => b.to_string(),
            Object::Null => "null".to_string(),
        };
    }
}
