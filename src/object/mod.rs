#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub struct Integer {
    pub value: i32,
}

impl Integer {
    pub fn inspect(&self) -> String {
        return self.value.to_string();
    }
}

#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub struct Boolean {
    pub value: bool,
}

impl Boolean {
    pub fn inspect(&self) -> String {
        return self.value.to_string();
    }
}

#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub enum Object {
    Integer(Integer),
    Boolean(Boolean),
    Null,
}

impl Object {
    pub fn inspect(&self) -> String {
        return match self {
            Object::Integer(i) => i.inspect(),
            Object::Boolean(b) => b.inspect(),
            Object::Null => "null".to_string(),
        };
    }
}
